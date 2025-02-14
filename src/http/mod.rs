pub mod request;  // Déclare le module request.rs
pub mod response; // Déclare le module response.rs
pub mod status;   // Déclare le module status.rs
pub mod headers;  // Déclare le module headers.rs

// Expose Request et Response pour les importer facilement dans d'autres modules
pub use request::Request;
pub use response::Response;
