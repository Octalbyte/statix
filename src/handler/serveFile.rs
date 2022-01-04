use infer;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::io::Error;
use tiny_http::{Header, Request, Response, StatusCode};
pub fn serveFile(rq: Request, path: &str) -> Result<(), Error> {
    let rs = File::open(Path::new(&("./".to_owned() + &path)));
    match rs {
        Err(reason) => {
            match reason.code {
                2 => {
                    // create function to handle 404...
                    let result = rq.respond(
                        Response::from_string(format!("Could not find {}", path))
                        .with_status_code(StatusCode(404))
                    );
                    return result;
                },
                _ => {}
            }

            let result = rq.respond(
                Response::from_string("Internal error").
                .with_status_code(StatusCode(502))
            );
            return result;
        }
        _ => {
            ();
        }
    }

    let kind = infer::get_from_path("./".to_owned() + &path).unwrap();
    match kind {
        Some(value) => {
            let kind = value.mime_type();
            let rs = rs.unwrap();
            let result = rq.respond(
                Response::from_file(rs)
                    .with_status_code(StatusCode(200))
                    .with_header(
                        Header::from_str(format!("Content-Type: {}", kind).as_str()).unwrap(),
                    ),
            );
            return result;
        }
        None => {
            let rs = rs.unwrap();
            let result = rq.respond(
                Response::from_file(rs)
                    .with_status_code(StatusCode(200))
                    .with_header(
                        Header::from_bytes(&b"Content-Type"[..], &b"application/octet-stream"[..])
                            .unwrap(),
                    ),
            );
            return result;
        }
    };
}
