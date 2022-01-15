extern crate colored;
extern crate tiny_http;

use clap::Parser;
use colored::*;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use tiny_http::{Server, ServerConfig, SslConfig};

mod handler;
mod lib;

const about: &str = "Simple CLI static file server";
const version: &str = "4.6.2";
const author: &str = "@Octalbyte";

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long, default_value = "127.0.0.1")]
    host: String,

    #[clap(short, long, default_value = "8080")]
    port: u32,

    #[clap(short, long, default_value = "cert.pem")]
    crt: String,

    #[clap(short, long, default_value = "key.pem")]
    key: String,

    #[clap(short, long)]
    ssl: bool,

    #[clap(short, long, default_value = "10")]
    threads: String,

    #[clap(long, default_value = "")]
    cors: String,
}

fn main() {
    let args = Args::parse();
    println!("Binding to {}:{}", args.host, args.port);
    let to_bind = format!("{}:{}", args.host, args.port);
    let argCors = args.cors.as_str();
    let cors = Arc::new(args.cors);

    let mut crt: Option<SslConfig> = None;

        if (args.ssl == true){
            crt = Some(SslConfig{
                certificate: lib::crt::public(args.crt),
                private_key: lib::crt::private(args.key)
            })
        }


    let server = Server::new(ServerConfig {
        addr: to_bind,
        ssl: crt,
    })
    .unwrap();
    let server = Arc::new(server);

    let mut guards = Vec::with_capacity(args.threads.parse::<usize>().unwrap());

    for _ in 0..args.threads.parse::<i32>().unwrap() {

        let server = server.clone();
        let cors = cors.clone();
        let guard = thread::spawn( move || {
            loop {
                let rq = server.recv().unwrap();

                //println!("{:?}", &rq);

                let output = format!("{:?}", &rq);

                let path = String::from(rq.url());
                let path = path.split("?");
                let path: Vec<&str> = path.collect();
                let path = path[0];

                if path.contains("../") || path.contains("\\") || path.contains(":") {
                    let _i = handler::badRequest(rq);
                    println!("{} -> {}", output, "500 Bad Request".red());
                    continue; //bad request
                }

                if Path::new(&("./".to_owned() + &path)).is_dir() {
                    if Path::new(&("./".to_owned() + &path + "/index.html")).exists() {
                        let _i = handler::serveFile(rq, &("./".to_owned() + &path + "/index.html"), &cors);
                        println!("{} -> {}", output, "200 Served index.html".green());
                        continue;
                    }
                    let _i = handler::serveFolder(rq, &path);
                    println!("{} -> {}", output, "200 Served Folder".green());
                    continue;
                }

                let _i = handler::serveFile(rq, &path, &cors);
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
