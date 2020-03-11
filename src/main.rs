use actix_web::{web, get, post, web::Form, web::Path, App, HttpServer, HttpResponse, Result};
use std::fs::{write, File};
use std::io::{Read, Error, Result as IOResult};
use serde::{Deserialize};
use std::env;
use actix_web::http::StatusCode;

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
async fn edit_form(web::Query(info): web::Query<EditFormQuery>) -> Result<HttpResponse> {
    let base_path = env::current_dir()?;
    println!("{}/{}", base_path.display(), info.path);

    let mut file = File::open(format!("{}/{}", base_path.display(), info.path))
        .expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("<form method='post' action='/edit'>\
        <input type='hidden' name='path' value='{}'/>\
        <textarea name='text'>{}</textarea>\
        <input type='submit' value='submit'/><form>", info.path, contents)))
}

#[post("/edit")]
async fn edit_submit_handler(form: Form<FormData>) -> Result<HttpResponse> {
    let base_path = env::current_dir()?;
    let file_path = format!("{}/{}", base_path.display(), form.path);
    let redirect_path = format!("/view{}", form.path);

    write(file_path, form.text.as_str())
        .expect("Unable to write file");

    Ok(HttpResponse::Ok()
        .status(StatusCode::FOUND)
        .header("location", redirect_path).body(""))
}

#[get("/view/{path:.*}")]
async fn index(path: Path<(String,)>) -> Result<String, Error> {
    let base_path = env::current_dir()?;
    let mut file = File::open(format!("{}/{}", base_path.display(), path.0))
        .expect("Unable to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    Ok(contents)
}

#[actix_rt::main]
async fn main() -> IOResult<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/resource2/index.html")
            )
            .service(index)
            .service(edit_form)
            .service(edit_submit_handler)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}