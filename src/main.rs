use std::net::{TcpListener, TcpStream};
use std::{thread, time};
use std::io::Read;
use std::io::Write;
use std::str;

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 { 
                    break;
                }
                stream.write(&read[0..n]).unwrap();

                let s = match str::from_utf8(&read[0..n]) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                println!("Received: {}", s);
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}

fn listen(hostname: &str, port: &str) {
    let address = hostname.to_owned() + ":" + port;
    let listener = TcpListener::bind(address).unwrap();

    println!("STARTED LISTENING");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}

fn send(hostname: &str, port: &str) -> std::io::Result<()> {
    let address = hostname.to_owned() + ":" + port;
    let mut stream = TcpStream::connect(address)?;

    println!("STARTED SENDING");

    loop {
        stream.write("Hello World!".as_bytes())?;
        thread::sleep(time::Duration::from_millis(2000));
    }
    Ok(())
}

fn main() {
    let hostname = "127.0.0.1";
    let port = "3000";

    let listen_thread = thread::spawn(move || {
        listen(hostname, port);
    });

    let send_thread = thread::spawn(move || {
        send(hostname, port);
    });

    let res1 = listen_thread.join();
    let res2 = send_thread.join();
}