use tiny_http::{Header, Request, Response, StatusCode};
//use std::io::error::Error::IoError;
pub fn badRequest(rq: Request) -> Result<(), IoError> {
    let result = rq.respond(
        Response::from_string("<html><body><h1>BAD REQUEST :(</h1></body></html>")
            .with_status_code(StatusCode(500))
            .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap()),
    );
    return result;
}
