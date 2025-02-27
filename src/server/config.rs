use std::fs;
use std::path::Path;

#[derive(Clone)]
pub struct ServerConfig {
    pub host: String,
    pub ports: Vec<u16>,
    pub routes: Vec<Route>,
    pub error_pages: Vec<(u16, String)>,
    pub max_body_size: usize,
}

#[derive(Clone)]
pub struct Route {
    pub path: String,
    pub methods: Vec<String>,
    pub root: String,
    pub cgi: Option<(String, String)>,
    pub default_file: Option<String>,
    pub directory_listing: bool,
}

#[derive(Clone)]
pub struct Config {
    pub servers: Vec<ServerConfig>,
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let mut servers = Vec::new();
        let mut current_server: Option<ServerConfig> = None;

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("server {") {
                current_server = Some(ServerConfig {
                    host: String::new(),
                    ports: Vec::new(),
                    routes: Vec::new(),
                    error_pages: Vec::new(),
                    max_body_size: 1048576,
                });
            } else if line == "}" && current_server.is_some() {
                servers.push(current_server.take().unwrap());
            } else if let Some(ref mut server) = current_server {
                let parts: Vec<&str> = line.split_whitespace().collect();
                match parts.get(0) {
                    Some(&"host") => server.host = parts[1].to_string(),
                    Some(&"ports") => server.ports = parts[1..].iter().map(|p| p.parse().unwrap()).collect(),
                    Some(&"max_body_size") => server.max_body_size = parts[1].parse()?,
                    Some(&"error_page") => server.error_pages.push((parts[1].parse()?, parts[2].to_string())),
                    Some(&"route") => {
                        let mut route = Route {
                            path: parts[1].to_string(),
                            methods: Vec::new(),
                            root: String::new(),
                            cgi: None,
                            default_file: None,
                            directory_listing: false,
                        };
                        let route_content = content.lines()
                            .skip_while(|l| !l.contains(&format!("route {}", parts[1])))
                            .skip(1)
                            .take_while(|l| !l.trim().is_empty() && !l.trim().starts_with('}'))
                            .collect::<Vec<_>>();
                        for r_line in route_content {
                            let r_parts: Vec<&str> = r_line.trim().split_whitespace().collect();
                            match r_parts.get(0) {
                                Some(&"methods") => route.methods = r_parts[1..].iter().map(|s| s.to_string()).collect(),
                                Some(&"root") => route.root = r_parts[1].to_string(),
                                Some(&"cgi") => route.cgi = Some((r_parts[1].to_string(), r_parts[2].to_string())),
                                Some(&"default_file") => route.default_file = Some(r_parts[1].to_string()),
                                Some(&"directory_listing") => route.directory_listing = r_parts[1] == "true",
                                _ => {}
                            }
                        }
                        server.routes.push(route);
                    }
                    _ => {}
                }
            }
        }
        Ok(Config { servers })
    }
}