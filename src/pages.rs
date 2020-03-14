use std::path::Path;

fn get_header() -> String {
    format!("<!DOCTYPE html>
        <html lang='en'>
        <head><link rel='stylesheet' type='text/css' href='/assets/main.css'/></head>
        <body id='page-container'>")
}

fn get_footer() -> String {
    format!("</body>")
}

pub fn edit(page_path: &str, page_contents: &str) -> String {
    format!(r#"
        {header}
        <div class='page-content'>
          <form id='edit' method='post' action='/edit'>
            <input type='hidden' name='path' value='{path}'/>
            <textarea name='text'>{contents}</textarea>
          <form>
        </div>
        <footer>
          <input form='edit' type='submit' value='Publish'/>
        </footer>
        {footer}"#, header=get_header(), path=page_path,
            contents=page_contents, footer=get_footer())
}

pub fn view(page_path: &str, page_contents: &str, edit_link: &str) -> String {
    let path_parts: Vec<&str> = page_path.split(".").collect();
    let path = Path::new(&path_parts[0]);
    let parent = path.parent().unwrap();

    let parent_html = if parent.eq(Path::new("index"))
    { format!("[ <a href='/view/{parent_link}.md'>&#8679; parent</a> ]",parent_link=parent.display()) }
    else {format!("")};
    format!(r#"{header}
            <div class='page-content'>
            {contents}
            </div>
            <footer>
              [ <a href='/view/index.md'>home</a> ]
              [ <a href='{edit_link}'>edit</a> ]
              {parent_html}
              [ <a href='#' onclick="n=prompt('Name'); if(n) window.location = '/view/{path}/' + n + '.md';">create subpage</a> ]
            </footer>
            {footer}"#, header=get_header(),
            contents=page_contents, edit_link=edit_link, parent_html=parent_html, path=path_parts[0], footer=get_header())
}
