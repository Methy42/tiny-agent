use clap::Parser;

mod args;
mod utils;
mod server;
mod client;

fn main() {
    let props = args::Args::parse();

    println!("{:?}", props);

    
    if props.client {
        client::init_client(props)
    } else {
        server::init_server(props);
    }

}
