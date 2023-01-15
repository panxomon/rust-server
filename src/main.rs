use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;



fn main() {
    println!("Hola !!!!");

    server()
}


fn server(){

    const HOST : &str = "127.0.0.1";
    const PORT : &str = "8080";

    let end_point : String = HOST.to_owned() + ":" +  PORT;


    let listener = TcpListener::bind(end_point).unwrap();

    println!("Web server is listening at port {}",PORT);     

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        
        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";    

    stream.write(response.as_bytes()).unwrap();

    stream.flush().unwrap();


}