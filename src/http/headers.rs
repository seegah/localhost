pub struct Headers;

impl Headers {
    pub fn get_content_length(headers: &[(String, String)]) -> Option<usize> {
        headers.iter()
            .find(|(k, _)| k.to_lowercase() == "content-length")
            .and_then(|(_, v)| v.parse().ok())
    }
}