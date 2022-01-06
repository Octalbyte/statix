extern crate tiny_http;
extern crate colored;

use clap::Parser;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use tiny_http::{Server, ServerConfig, SslConfig};
use colored::*;

mod handler;
mod lib;

const about: &str = "Simple CLI static file server";
const version: &str = "4.0.3";
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

    #[clap(short, long, default_value = "10")]
    threads: String,
}

fn main() {
    let args = Args::parse();
    println!("{}:{}", args.host, args.port);
    let to_bind = format!("{}:{}", args.host, args.port);

    let mut crt: Option<SslConfig> = None;
    /*
        if (args.crt != "None"){
            crt = Some(SslConfig{
                certificate: lib::crt::public(args.crt),
                private_key: lib::crt::private(args.crt)
            })
        }
    */

    let server = Server::new(ServerConfig {
        addr: to_bind,
        ssl: crt,
    })
    .unwrap();
    let server = Arc::new(server);

    let mut guards = Vec::with_capacity(args.threads.parse::<usize>().unwrap());

    for _ in 0..args.threads.parse::<i32>().unwrap() {
        //change this so user can choose threads
        let server = server.clone();

        let guard = thread::spawn(move || {
            loop {
                let rq = server.recv().unwrap();

                //println!("{:?}", &rq);

                let mut output = format!("{:?}", &rq);

                let path = String::from(rq.url());

                if path.contains("../") || path.contains("\\") || path.contains(":") {
                    let i = handler::badRequest(rq);
                    println!("{} -> {}", output, "500 Bad Request".red());
                    continue; //bad request
                }

                if Path::new(&("./".to_owned() + &path)).is_dir() {
                    let i = handler::serveFolder(rq, &path);
                    println!("{} -> {}", output, "200 Served Folder".green());
                    continue;
                }

                let _i = handler::serveFile(rq, &path);

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
