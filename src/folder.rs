
pub fn serveFolder(rq: Request, path: &str){
    let entries = fs::read_dir(Path::new(&("./".to_owned()+ &path)));
    let entries = match entries {
        Err(why) => {
            fs::read_dir("./").unwrap()
        },
        Ok(value) => {
            value
        }

    };
    let mut TheResponse: String = String::from(
        format!("<html><body><h1>Scanning directory {}</h1></br>", &path)
    );


    for entry in entries {
        let path = entry.unwrap().path();
        let n = path.as_path().to_str();
        let mut i = String::from("");
        let n = match n{
        None =>  {
            "404"
        },
        Some(value) => {
            i = String::from(value);
            i.remove(0);
            i.remove(0);
            i.remove(0);
            i.as_str()
        }
    };
        let a: Vec<&str> =  n.split("/").collect();
        let a = a.last().unwrap();
        TheResponse = TheResponse+"<a href = "+n+" >"+a+"</a>"+"</br>";

    }
    rq.respond(
        Response::from_string(TheResponse)
        .with_status_code(
            StatusCode(200)
         )
         .with_header(
            Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap()
         )
    );
}