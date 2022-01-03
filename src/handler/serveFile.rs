use infer;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use tiny_http::{Header, Request, Response, StatusCode};

pub fn serveFile(rq: Request, path: &str) {
    let rs = File::open(Path::new(&("./".to_owned() + &path)));
    match rs {
        Err(reason) => {
            rq.respond(Response::from_string(format!("{:#?}", reason)));
            return;
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
            rq.respond(
                Response::from_file(rs)
                    .with_status_code(StatusCode(200))
                    .with_header(
                        Header::from_str(format!("Content-Type: {}", kind).as_str()).unwrap(),
                    ),
            );
        }
        None => {
            let rs = rs.unwrap();
            rq.respond(
                Response::from_file(rs)
                    .with_status_code(StatusCode(200))
                    .with_header(
                        Header::from_bytes(&b"Content-Type"[..], &b"application/octet-stream"[..])
                            .unwrap(),
                    ),
            );
        }
    };
}