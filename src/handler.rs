//include all the handlers here
mod folder;
pub use folder::serveFolder as serveFolder;
mod badRequest;
pub use badRequest::badRequest as badRequest;
mod serveFile;
pub use serveFile::serveFile as serveFile;
