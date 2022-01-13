extern crate colored;

use colored::*;
use infer;
use std::fs::File;
use std::io::Error;
use std::path::Path;
use std::str::FromStr;
use std::fs;
use std::io::ErrorKind;
use tiny_http::{Header, Request, Response, StatusCode};

pub fn serveFile(rq: Request, path: &str) -> Result<(), Error> {
    let rs = File::open(Path::new(&("./".to_owned() + &path)));
    let output = format!("{:?}", &rq);
    match rs {
        Err(reason) => {
            //rq.respond(Response::from_string(format!("{:?}", reason)));
            //return Ok(());
            match reason.kind() {
                ErrorKind::NotFound => {
                    // create function to handle 404...

                    if Path::new("./404.html").exists() {
                        let result = serveFile(rq, "./404.html");
                        println!("{} -> {}", output, "404 Not Found".red());
                        return result;
                    }

                    let result = rq.respond(
                        Response::from_string(format!("Could not find {}", path))
                            .with_status_code(StatusCode(404)),
                    );
                    println!("{} -> {}", output, "404 Not Found".red());

                    return result;
                }
                _ => {}
            }

            let result = rq
                .respond(Response::from_string("Internal error").with_status_code(StatusCode(502)));
            let internal = format!("{:?}", reason);
            println!(
                "{} -> {} -> {}",
                output,
                "502 Internal Error".red(),
                internal.bold()
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
            let length = fs::metadata(&path).unwrap().len();
            let result = rq.respond(
                Response::from_file(rs)
                    .with_status_code(StatusCode(200))
                    .with_header(
                        Header::from_str(format!("Content-Type: {}", kind).as_str()).unwrap()
                    )
                    .with_header(
                        Header::from_str(format!("Content-Length: {}", length).as_str()).unwrap()
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
