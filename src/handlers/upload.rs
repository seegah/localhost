use crate::http::{Request, Response, Status};
use crate::utils::file;
use std::path::Path;

pub fn handle_upload(request: &Request, upload_dir: &Path) -> Response {
    if request.method != "POST" {
        return Response::new(Status::new(405));
    }

    let filename = request.headers.iter()
        .find(|(k, _)| k.to_lowercase() == "content-disposition")
        .and_then(|(_, v)| v.split("filename=\"").nth(1))
        .and_then(|v| v.split('"').next())
        .unwrap_or("uploaded_file");

    let filepath = upload_dir.join(filename);
    if let Err(_) = file::write_file(&filepath, &request.body) {
        return Response::new(Status::new(500));
    }

    let mut response = Response::new(Status::new(200));
    response.body = b"File uploaded successfully".to_vec();
    response
}