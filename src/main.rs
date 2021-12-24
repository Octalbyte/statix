extern crate tiny_http;
use tiny_http::{
    Server,
    Response
};
extern crate argparse;

use argparse::{
    ArgumentParser, 
    StoreTrue, 
    Store
};


fn main() {
    
    let mut server = Server::http();

    println!("Hello, world!");

}
