use std::io::Error;
use tiny_http::{
    Request, Response, HeaderField, Header, StatusCode
    
};

pub fn unauthorized(rq: Request) -> Result<(), Error> {
    let result = rq.respond(
        Response::from_string("401 Unauthorized")
            .with_status_code(StatusCode(401))
            .with_header(Header::from_bytes(&b"WWW-Authenticate"[..], &b"Basic realm=\"Access website\""[..]).unwrap()),
    );
    return result;
}