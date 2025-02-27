use crate::http::{Request, Response, Status};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn handle_cgi_request(request: &Request, path: &Path, command: &str) -> Response {
    let mut child = match Command::new(command)
        .arg(path)
        .env("REQUEST_METHOD", &request.method)
        .env("PATH_INFO", path.to_str().unwrap_or(""))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Ok(child) => child,
        Err(_) => return Response::new(Status::new(500)),
    };

    if let Some(stdin) = child.stdin.as_mut() {
        if let Err(_) = std::io::Write::write_all(stdin, &request.body) {
            return Response::new(Status::new(500));
        }
    }

    match child.wait_with_output() {
        Ok(output) if output.status.success() => {
            let mut response = Response::new(Status::new(200));
            response.body = output.stdout;
            response.headers.push(("Content-Type".to_string(), "text/html".to_string()));
            response
        }
        _ => Response::new(Status::new(500)),
    }
}