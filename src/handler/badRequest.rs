use std::io::Error;
use tiny_http::{Header, Request, Response, StatusCode};

#[allow(non_snake_case)]
pub fn badRequest(rq: Request) -> Result<(), Error> {
    let result = rq.respond(
        Response::from_string("<html><body><h1>BAD REQUEST :(</h1></body></html>")
            .with_status_code(StatusCode(500))
            .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap()),
    );
    return result;
}
