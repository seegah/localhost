use std::collections::HashMap;

pub struct Headers {
    headers: HashMap<String, String>,
}

impl Headers {
    /// Crée une nouvelle instance de `Headers`
    pub fn new() -> Self {
        Headers {
            headers: HashMap::new(),
        }
    }

    /// Définit un en-tête
    pub fn set(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    /// Récupère un en-tête
    pub fn get(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    /// Convertit les en-têtes en une chaîne de caractères au format HTTP
    pub fn to_string(&self) -> String {
        self.headers
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect::<Vec<String>>()
            .join("\r\n")
    }
}
