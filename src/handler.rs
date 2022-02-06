//include all the handlers here
#[allow(non_snake_case)]
mod folder;
pub use folder::serveFolder;
#[allow(non_snake_case)]
mod badRequest;
pub use badRequest::badRequest;
#[allow(non_snake_case)]
mod serveFile;
pub use serveFile::serveFile;
#[allow(non_snake_case)]
mod unauthorized;
pub use unauthorized::unauthorized;