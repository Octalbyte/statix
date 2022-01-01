//include all the handlers here

use tiny_http::{
    Response,
    Request,
    Header,
    StatusCode
};
mod folder;
pub use folder::serveFolder as serveFolder;

pub fn badRequest (rq: Request){
    rq.respond(
        Response::from_string("<html><body><h1>BAD REQUEST :(</h1></body></html>")
        .with_status_code(
         StatusCode(500)
      )
      .with_header(
         Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap()
      )
     );
}