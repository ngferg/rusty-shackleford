use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Unable to get port");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line: Vec<&str> = http_request[0].split(" ").collect();
    let response: String;

    if request_line.len() != 3 {
        response = create_response("500", "/500.html");
    } else {
        let method = request_line[0];
        let file_path = request_line[1];

        if method == "GET" && (file_path == "/") {
            response = create_response("200", "/hello.html");
        } else {
            response = create_response("200", file_path);
        }
    }

    stream.write_all(response.as_bytes()).unwrap();

    println!("Request: {http_request:#?}");
    println!("Responding with: {response}");
}

fn create_response(status_code: &str, file_path: &str) -> String {
    let mut status_line = format!("HTTP/1.1 {status_code} OK");
    
    let formatted_path = format!("static{file_path}");
    match fs::exists(&formatted_path) {
        Ok(true) => {
            let contents = fs::read_to_string(&formatted_path).unwrap_or_else(|_| { 
                status_line = "HTTP/1.1 404 OK".to_string();
                fs::read_to_string("static/404.html").expect("404 file must exist")
            });
            let length = contents.len();
            let content_type = "text/html";
            let headers =
                format!("Content-Type: {content_type}\r\nContent-Length: {length}");
        
            format!("{status_line}\r\n{headers}\r\n\r\n{contents}")
        },
        _ => create_response("404", "/404.html")
    }

}
