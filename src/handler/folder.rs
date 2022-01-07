use std::fs;
use std::path::Path;
use tiny_http::{Header, Request, Response, StatusCode};
use std::io::Error;
pub fn serveFolder(rq: Request, path: &str) -> Result<(), Error> {
    let entries = fs::read_dir(Path::new(&("./".to_owned() + &path)));
    let entries = match entries {
        Err(_why) => fs::read_dir("./").unwrap(),
        Ok(value) => value,
    };
    let mut TheResponse: String = String::from(format!(
        "<html><body><h1>Scanning directory {}</h1></br>",
        &path
    ));
    let p = &("./".to_owned() + &path);
    let parent = Path::new(p).parent();
    //println!("{}", parent);

    match parent {
        Some(parent) => {
            match parent.to_str() {
                Some(parent) => {
                    TheResponse = TheResponse + "<a href = " + parent + " > <b> .. (parent folder) </b> </a></br>";
                },
                None => {

                }
            }

        },
        None => {

        }
    }

    for entry in entries {
        let path = entry.unwrap().path();
        let n = path.as_path().to_str();
        let mut i = String::from("");
        let n = match n {
            None => "Unlistable folder",
            Some(value) => {
                i = String::from(value);
                i.remove(0);
                i.remove(0);
                i.remove(0);
                i.as_str()
            }
        };
        let a: Vec<&str> = n.split("/").collect();
        let a = a.last().unwrap();
        TheResponse = TheResponse + "<a href = " + n + " >" + a + "</a>" + "</br>";
    }
    let result = rq.respond(
        Response::from_string(TheResponse)
            .with_status_code(StatusCode(200))
            .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap()),
    );
    return result;
}
