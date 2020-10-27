use std::path::PathBuf;

use comrak::{markdown_to_html, ComrakOptions};
use wasmcompute_lib::http::{
    http_serve, types::Mime, types::StatusCode, WasmRequest, WasmResponse,
};

fn html404(mut res: WasmResponse) -> WasmResponse {
    // can't error out right now -_- guess i'll just make the 404 response
    let notfound = std::fs::read_to_string("/app/templates/404.html").unwrap();
    res.body(notfound.as_bytes().to_owned());
    res.content_type(Mime::HTML);
    return res;
}

fn handle(req: WasmRequest) -> WasmResponse {
    let mut res = WasmResponse::new(StatusCode::Ok);
    let request_path = req.url.path;
    let markdown_path = if request_path == "/" || request_path == "" {
        PathBuf::from("/app/README.md")
    } else {
        let mut path = PathBuf::from("/app").join(format!("{}", request_path));
        if path.is_dir() {
            path = path.join("README.md");
            if !path.exists() {
                return html404(res);
            }
            path
        } else {
            if path.extension() == None {
                path.set_extension("md");
            }
            if !path.exists() {
                return html404(res);
            }
            path
        }
    };
    println!("Rendering file {:?}", markdown_path);
    let layout = std::fs::read_to_string("/app/templates/template.html").unwrap();
    let markdown = std::fs::read_to_string(&markdown_path).unwrap();
    let content = markdown_to_html(&markdown, &ComrakOptions::default());
    let html = layout.replace("{{content}}", &content);
    res.body(html.as_bytes().to_owned());
    res.content_type(Mime::HTML);
    res
}

fn main() -> wasmcompute_lib::error::Result<()> {
    match http_serve(handle) {
        Ok(_) => println!("User was serviced successfully"),
        Err(e) => println!("The application errored '{}' we paniced", e),
    };
    Ok(())
}
