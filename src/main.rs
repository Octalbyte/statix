use infer;
use std::path::Path;
extern crate tiny_http;
use std::fs::File;
use std::fs;
use tiny_http::{
    Server,
    Response,
    ServerConfig,
    SslConfig,
    Request,
    Header,
    StatusCode,
    HeaderField
};

use ascii;

use std::rc::Rc;

use std::sync::Arc;
mod lib; //must be fixed

const about: &str = "CLI simple static file server";
const version: &str = "3.0.0";
const author: &str = "@Octalbyte";

use clap::Parser;

use std::thread;

#[derive(Parser, Debug)]
#[clap(about,version , author)]
struct Args {

    #[clap(short, long, default_value = "127.0.0.1")]
    host: String,

    #[clap(short, long, default_value = "8080")]
    port: u32,

    #[clap(short, long, default_value = "None")]

    crt: String

}



fn main() {

    let args = Args::parse();

    println!("{}:{}", args.host, args.port);

    let to_bind = format!("{}:{}", args.host, args.port);

    let mut crt: Option<SslConfig> = None;
/*
    if (args.crt != "None"){
        crt = Some(SslConfig{
            certificate: lib::Crt::public(args.crt),
            private_key: lib::Crt::private(args.crt)
        })
    }
*/

let server = Server::new(ServerConfig {
        addr: to_bind,
        ssl: crt
    }).unwrap();
let server = Arc::new(server);
let mut guards = Vec::with_capacity(5);

for _ in 0 .. 5 { //change this so user can choose threads
    let server = server.clone();

    let guard = thread::spawn(move || {
       'outer: while true {
            let rq = server.recv().unwrap();

            println!("{:?}", &rq);
            let path = String::from(rq.url());

            if path.contains("../") || path.contains("\\") || path.contains(":") {
               rq.respond(
                   Response::from_string("<html><body><h1>BAD REQUEST :(</h1></body></html>")
                   .with_status_code(
                    StatusCode(500)
                 )
                 .with_header(
                    Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap()
                 )
                );
                continue; //bad request
            }

            let b: bool = Path::new(&("./".to_owned()+ &path)).is_dir();

            if b {
                let entries = fs::read_dir(Path::new(&("./".to_owned()+ &path)));
                let entries = match entries {
                    Err(why) => {
                        fs::read_dir("./").unwrap()
                    },
                    Ok(value) => {
                        value
                    }

                };
                let mut TheResponse: String = String::from(
                    format!("<html><body><h1>Scanning directory {}</h1></br>", &path)
                );


                for entry in entries {
                    let path = entry.unwrap().path();
                    let n = path.as_path().to_str();
                    let mut i = String::from("");
                    let n = match n{
                    None =>  {
                        "404"
                    },
                    Some(value) => {
                        i = String::from(value);
                        i.remove(0);
                        i.remove(0);
                        i.remove(0);
                        i.as_str()
                    }
                };
                    let a: Vec<&str> =  n.split("/").collect();
                    let a = a.last().unwrap();
                    TheResponse = TheResponse+"<a href = "+n+" >"+a+"</a>"+"</br>";
                    //let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
                    //assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

                }
                rq.respond(
                    Response::from_string(TheResponse)
                    .with_status_code(
                        StatusCode(200)
                     )
                     .with_header(
                        Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap()
                     )
                );
                continue;
            }

            let rs = File::open(Path::new(&("./".to_owned()+&path)));
            match rs {
                Err(reason) => {
                    rq.respond(Response::from_string(format!("{:#?}", reason)));
                    continue;
                },
                _ => {
                    ();
                }
            }
            let kind = infer::get_from_path("./".to_owned()+&path).unwrap();

            let kind = match kind {
                Some(value) => {
                    value
                },
                None => {
                    
                }
            }
            let kind = kind.mime_type();
           let rs = rs.unwrap();
           rq.respond(
               Response::from_file(rs)
               .with_status_code(
                StatusCode(200)
             )
             .with_header(
                Header::from_bytes(&b"Content-Type"[..], &b"text/html"/*+kind*/[..]).unwrap()
             )
            );

        }
    });

    guards.push(guard);
}

for guard in guards {

    let rs = guard.join();

    match rs {
        Err(e) => {
            println!("{:#?}", e);
        },
        _ => {
            ();
        }

    }
}
}






