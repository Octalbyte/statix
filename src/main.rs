extern crate tiny_http;

use clap::Parser;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use tiny_http::{Request, Server, ServerConfig, SslConfig};

mod handler;
mod lib;

const about: &str = "Simple CLI static file server";
const version: &str = "3.0.0";
const author: &str = "@Octalbyte";

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long, default_value = "127.0.0.1")]
    host: String,

    #[clap(short, long, default_value = "8080")]
    port: u32,

    #[clap(short, long, default_value = "None")]
    crt: String,
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
        ssl: crt,
    })
    .unwrap();
    let server = Arc::new(server);

    let mut guards = Vec::with_capacity(5);

    for _ in 0..5 {
        //change this so user can choose threads
        let server = server.clone();

        let guard = thread::spawn(move || {
            'outer: while true {
                let rq = server.recv().unwrap();

                println!("{:?}", &rq);
                let path = String::from(rq.url());

                if path.contains("../") || path.contains("\\") || path.contains(":") {
                    handler::badRequest(rq);
                    continue; //bad request
                }

                if Path::new(&("./".to_owned() + &path)).is_dir() {
                    handler::serveFolder(rq, &path);
                    continue;
                }

                handler::serveFile(rq, &path);
            }
        });

        guards.push(guard);
    }

    for guard in guards {
        let rs = guard.join();

        match rs {
            Err(e) => {
                println!("{:#?}", e);
            }
            _ => {
                ();
            }
        }
    }
}
