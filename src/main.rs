extern crate tiny_http;
use tiny_http::{
    Server,
    Response
};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap("CLI simple static file server", "0.1.0", "@J-P-S-O")]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}



fn main() {
    let mut host = "127.0.0.1";
    let mut port = 8080;
   
    // let mut server = Server::http();

    println!("{}:{}", host, port);

}
