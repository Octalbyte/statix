//include all the handlers here
mod folder;
pub use folder::serveFolder;
mod badRequest;
pub use badRequest::badRequest;
mod serveFile;
pub use serveFile::serveFile;
mod unauthorized;
pub use unauthorized::unauthorized;