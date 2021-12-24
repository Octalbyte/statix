extern crate tiny_http;
use tiny_http::{
    Server,
    Response,
    ServerConfig,
    SslConfig
};

const about: &str = "CLI simple static file server";
const version: &str = "0.1.0";
const author: &str = "@J-P-S-O";

use clap::Parser;

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

    let mut server = Server::new(ServerConfig {
        addr: to_bind, 
        ssl: crt
    });

}
