#![allow(unused_must_use)]

extern crate colored;
extern crate tiny_http;
extern crate base64;
extern crate ascii;
extern crate ctrlc;

use clap::Parser;
use istor::istor;
use colored::*;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use tiny_http::{Server, ServerConfig, SslConfig, StatusCode, Response, HeaderField};
use ascii::AsciiStr;

mod handler;
mod lib;

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
const about: &str = "Simple CLI static file server";
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
const version: &str = "4.8.3";
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
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

    #[clap(long)]
    blocktor: bool,

    #[clap(long, short, default_value = "")]
    username: String,

    #[clap(long, default_value = "")]
    pwd: String,
}

fn main() {
    println!("Statix v{} made by {}", &version.bold(), &author.bold());
    let args = Args::parse();
    println!("Binding to {}:{}", args.host, args.port);
    let to_bind = format!("{}:{}", args.host, args.port);
    let cors = Arc::new(args.cors);
    let mut restricted = Arc::new(false);
    let pwd = Arc::new(args.pwd);
    let mut username = Arc::new(String::from(""));
    if args.username != "" {
        username = Arc::new(args.username);
        restricted = Arc::new(true);
    }
    
    let block_tor = Arc::new(args.blocktor);

    let mut crt: Option<SslConfig> = None;

        if args.ssl {
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
        let restricted = restricted.clone();
        let username = username.clone();
        let pass = pwd.clone();
        let blocktor = Arc::clone(&block_tor);
        let guard = thread::spawn( move || {
            'outer: loop {
                let rq = server.recv().unwrap();
                let output = format!("{:?}", &rq);

		let headers = rq.headers();
                let mut auth = None;
                for i in headers.iter() {
                    if i.field == HeaderField::from_bytes("Authorization".as_bytes().to_vec()).unwrap() {
                        let _wrds = &i.value;
                        let wrds: Vec<&AsciiStr> = i.value.split(ascii::AsciiChar::Space).collect();
                        if *restricted && wrds.len() < 2 {
                            handler::unauthorized(rq);
                            println!("{} -> {}", output, "401 Unauthorized".red());
                            continue 'outer;
                            
                        }
                        if *restricted && wrds[0] != "Basic" {
			    handler::unauthorized(rq);
                            println!("{} -> {}", output, "401 Unauthorized".red());
                            continue 'outer;
                        }
                        auth = Some(wrds[1]);
                    }
                }

                if *restricted {
                    match auth {
                        None => {
                            handler::unauthorized(rq);
                            println!("{} -> {}", output, "401 Unauthorized".red());
                            continue 'outer;
                        }
                        Some(authtry) => {
                            if base64::encode(format!("{}:{}", username, pass).as_bytes()) == String::from(authtry.as_str()) {
                        } else {
			    handler::unauthorized(rq);
                            println!("{} -> {}", output, "401 Unauthorized".red());
                            continue 'outer;
                        }

                        }
                    }
                }

                if *blocktor {
                    let str_to_be_checked = format!("{}", &rq.remote_addr());
                    let str_to_be_checked =  String::from(str_to_be_checked);
                    let str_to_be_checked = str_to_be_checked.split(":");
                    let str_to_be_checked: Vec<&str> = str_to_be_checked.collect();
                    let str_to_be_checked = String::from(str_to_be_checked[0]);
                    let str_to_be_checked = str_to_be_checked.as_str();

                    if istor::istor(str_to_be_checked, false){
                        println!("{} -> {}", output, "Blocked TOR request".red());
                       	rq.respond(
                            Response::from_string("You can't use Tor here ¯\\_(ツ)_/¯")
                            .with_status_code(StatusCode(500))
                        );
                        continue;
                    }
                }

                let path = String::from(rq.url());
                let path = path.split("?");
                let path: Vec<&str> = path.collect();
                let path = path[0];

                if path.contains("../") || path.contains("\\") || path.contains(":") {
                    let _i = handler::badRequest(rq);
                    println!("{} -> {}", output, "500 Bad Request".red());
                    continue; 
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
    ctrlc::set_handler(move || {
	println!("Exiting...");
	std::process::exit(0);	
	});
}
