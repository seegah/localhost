use crate::http::{Headers, Request, Response, Status};
use crate::server::config::{Config, Route};
use crate::handlers::{static_file, cgi, error_pages, upload};
use std::path::{Path, PathBuf};
use std::fs;

pub struct Router {
    config: Config,
}

impl Router {
    pub fn new(config: &Config) -> Self {
        Router {
            config: config.clone(),
        }
    }

    pub fn route(&self, request: &Request) -> Response {
        let server_config = self.config.servers.first()
            .unwrap_or_else(|| panic!("No server configuration available"));

        // Use version to ensure HTTP/1.1 compatibility
        if request.version != "HTTP/1.1" {
            return error_pages::get_error_page(400, server_config);
        }

        // Handle upload route specifically
        if request.path.starts_with("/upload") && request.method == "POST" {
            return upload::handle_upload(request, Path::new("uploads"));
        }

        let matched_route = server_config.routes.iter()
            .find(|route| request.path.starts_with(&route.path))
            .or_else(|| server_config.routes.iter().find(|route| route.path == "/"));

        match matched_route {
            Some(route) => self.handle_route(request, route, server_config),
            None => error_pages::get_error_page(404, server_config),
        }
    }

    fn handle_route(&self, request: &Request, route: &Route, server_config: &crate::server::config::ServerConfig) -> Response {
        if !route.methods.is_empty() && !route.methods.contains(&request.method) {
            return error_pages::get_error_page(405, server_config);
        }

        // Use headers to check content length
        if let Some(len) = Headers::get_content_length(&request.headers) {
            if len > server_config.max_body_size {
                return error_pages::get_error_page(413, server_config);
            }
        }

        let relative_path = request.path.strip_prefix(&route.path).unwrap_or(&request.path);
        let mut full_path = PathBuf::from(&route.root);
        full_path.push(relative_path.trim_start_matches('/'));

        if full_path.is_dir() {
            if let Some(default_file) = &route.default_file {
                full_path.push(default_file);
            } else if route.directory_listing {
                return self.handle_directory_listing(&full_path);
            } else {
                return error_pages::get_error_page(403, server_config);
            }
        }

        if let Some((ext, cmd)) = &route.cgi {
            if full_path.extension().and_then(|e| e.to_str()) == Some(ext) {
                return cgi::handle_cgi_request(request, &full_path, cmd);
            }
        }

        self.handle_static_file(&full_path, server_config)
    }

    fn handle_static_file(&self, path: &Path, server_config: &crate::server::config::ServerConfig) -> Response {
        match static_file::serve_file(path) {
            Ok(response) => response,
            Err(_) => error_pages::get_error_page(404, server_config),
        }
    }

    fn handle_directory_listing(&self, path: &Path) -> Response {
        let mut response = Response::new(Status::new(200));
        let mut html = String::from("<html><body><h1>Directory Listing</h1><ul>");

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                let name = entry.file_name().to_string_lossy().into_owned();
                html.push_str(&format!("<li>{}</li>", name));
            }
        }

        html.push_str("</ul></body></html>");
        response.body = html.into_bytes();
        response.headers.push(("Content-Type".to_string(), "text/html".to_string()));
        response
    }
}

