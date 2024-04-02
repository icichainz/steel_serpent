use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use clap::{App,Arg};
// Handle incoming connection and client messages.
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();   
    let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    // receive arguments from command line.
    let cmdargs =  App::new("Steel Serpent")
    .arg(
        Arg::with_name("port")
        .short("p")
        .long("port")
        .value_name("value")
        .help("specifies the app server")
        .takes_value(true),
    ).get_matches();
    // Create the listner and bind the address.
    let port = cmdargs.value_of("port").unwrap_or("8080") ;

  
    
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address).expect("Failed to bind address");
    println!("Server listening on http://127.0.0.1:{}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}