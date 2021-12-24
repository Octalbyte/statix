extern crate tiny_http;
use tiny_http::{
    Server,
    Response
};

const about: &str = "CLI simple static file server";
const version: &str = "0.1.0";
const author: &str = "@J-P-S-O";

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about,version , author)]
struct Args {

    #[clap(short, long, default_value_t = "127.0.0.1")]
    host: String,

    #[clap(short, long, default_value_t = 8080)]
    port: u32,
}



fn main() {
    let mut host = "127.0.0.1";
    let mut port = 8080;
   
    // let mut server = Server::http();
    let args = Args::parse();

    println!("{}:{}", args.host, args.port);

}
