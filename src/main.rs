
use std::path::Path;
extern crate tiny_http;
use std::fs::File;
use tiny_http::{
    Server,
    Response,
    ServerConfig,
    SslConfig,
    Request
};

use std::rc::Rc;

use std::sync::Arc;
mod lib; //must be fixed

const about: &str = "CLI simple static file server";
const version: &str = "0.1.0";
const author: &str = "@J-P-S-O";

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
        loop {
            let rq = server.recv().unwrap();

            println!("{:?}", &rq);
            let path = String::from(rq.url());

            if path.contains("../"){
                continue; //bad request
            } else {
                //println!("Safe request: {}", path);
            }
            let rs = File::open(Path::new(&("./".to_owned()+&path)));
            match rs {
                Err(reason) => {
                    rq.respond(Response::from_string(format!("{:#?}", reason)));
                    break;
                },
                _ => {
                    ();
                }
            }
            
           let rs = rs.unwrap();
           rq.respond(Response::from_file(rs));

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






