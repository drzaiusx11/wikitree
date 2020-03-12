use actix_web::{web, get, post, web::Form, web::Path, App, HttpServer, HttpResponse, Result};
use actix_files as fs;
use std::fs::{write, create_dir_all, File};
use std::io::{Read, Error, Result as IOResult};
use serde::{Deserialize};
use std::env;
use std::path::Path as FilePath;
use actix_web::http::StatusCode;
use markdown;

#[derive(Deserialize)]
struct FormData {
    path: String,
    text: String,
}

#[derive(Deserialize)]
struct EditFormQuery {
    path: String
}

#[get("/edit")]
async fn edit_form(web::Query(q): web::Query<EditFormQuery>) -> Result<HttpResponse> {
    let file_path = get_file_path(&q.path)?;
    let mut contents = format!("## title");

    if FilePath::new(&file_path).exists() {
        let mut file = File::open(file_path)
            .expect("Unable to open file");

        contents.clear();
        file.read_to_string(&mut contents).expect("unable to read from file");
    }

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("<form method='post' action='/edit'>\
        <input type='hidden' name='path' value='{}'/>\
        <textarea style='resize: none; width: 100%; height: calc(100% - 50px);' \
        name='text'>{}</textarea>\
        <input type='submit' value='submit'/><form>", q.path, contents)))
}

fn get_file_path(rel_path: &str) -> Result<String, Error> {
    let base_path = env::current_dir()?;
    Ok(format!("{}/{}", base_path.display(), rel_path))
}

fn get_view_url(rel_path: &str) -> String {
    format!("/view/{}", rel_path)
}

fn get_edit_url(rel_path: &str) -> String {
    format!("/edit?path={}", rel_path)
}

#[post("/edit")]
async fn edit_submit_handler(form: Form<FormData>) -> Result<HttpResponse, Error> {
    let file_path = get_file_path(&form.path)?;
    let redirect_url = get_view_url(&form.path);

    let path = FilePath::new(&file_path);

    if !path.exists() {
        let parent = path.parent().unwrap();
        create_dir_all(parent.as_os_str())?;
    }

    write(file_path, form.text.as_str())
        .expect("Unable to write file");

    Ok(HttpResponse::Ok()
        .status(StatusCode::FOUND)
        .header("location", redirect_url).body(""))
}

#[get("/view/{path:.*}")]
async fn index(path: Path<(String,)>) -> Result<HttpResponse, Error> {
    let file_path = get_file_path(&path.0)?;
    let edit_url = get_edit_url(&path.0);

    if FilePath::new(&file_path).exists() {
        let mut file = File::open(file_path)
            .expect("Unable to open file");

        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");

        let md = markdown::to_html(&contents);
        Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("<!DOCTYPE html>
                 <html lang='en'><head><link rel='stylesheet' type='text/css' href='/assets/fonts.css'/></head>
                 <body>\
            {} <br><a href='{}'>edit</a></body></html>", md, edit_url)))
    }
    else {
        Ok(HttpResponse::Ok()
            .status(StatusCode::FOUND)
            .header("location", edit_url).body(""))
    }
}

#[actix_rt::main]
async fn main() -> IOResult<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(edit_form)
            .service(edit_submit_handler)
            .service(
                // static files
                fs::Files::new("/assets", "./static/").index_file("fonts.css"),
            )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}