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
    let mut host = "127.0.0.1"
    let mut port = 8080
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Host a static file webserver");
        
        ap.refer(&mut name)
            .add_option(
                &["--host"], 
                Store,
                "Host to bind"
        );
        ap.refer(&mut port)
            .add_option(
                &["--port"],
                Store,
                "Port to listen"
        )
        ap.parse_args_or_exit();
    }
    
    // let mut server = Server::http();

    println!("{}:{}", host, port);

}
