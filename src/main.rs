use std::process::exit;

use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: SubPrograms,
}

#[derive(Subcommand, Debug)]
enum SubPrograms {
    Server {},
    Send { send: String },
    SendFile { file: String },
}

#[tokio::main]
async fn main() {
    let Cli { command } = Cli::parse();
    match command {
        SubPrograms::Server {} => {
            let listener = UnixListener::bind("/tmp/example.socket").unwrap();

            tokio::spawn(async move {
                tokio::signal::ctrl_c().await.unwrap();
                tokio::fs::remove_file("/tmp/example.socket").await.unwrap();
                exit(0);
            });

            loop {
                match listener.accept().await {
                    Ok((mut stream, _socket)) => {
                        let cred = stream.peer_cred().unwrap();
                        println!("New client. UID: {}, GID: {}", cred.uid(), cred.gid());
                        let mut content = String::new();
                        let size = stream.read_to_string(&mut content).await.unwrap();
                        println!("content size: {}, \ncontent: \n{}", size, content);
                        stream.write_all("Received.".as_bytes()).await.unwrap();
                        stream.shutdown().await.unwrap();
                    }
                    Err(err) => {
                        println!("Failed to connect {:#?}", err)
                    }
                }
            }
        }
        SubPrograms::Send { send } => {
            let mut socket = UnixStream::connect("/tmp/example.socket").await.unwrap();
            socket.write_all(send.as_bytes()).await.unwrap();
            socket.shutdown().await.unwrap();
            let mut response = String::new();
            let _size = socket.read_to_string(&mut response).await.unwrap();
            println!("Response: {}", response);
        }
        SubPrograms::SendFile { file } => {
            let mut socket = UnixStream::connect("/tmp/example.socket").await.unwrap();
            let content = fs::read_to_string(file).await.unwrap();
            socket.write_all(content.as_bytes()).await.unwrap();
            socket.shutdown().await.unwrap();
            println!("All data sended");
            let mut response = String::new();
            let _size = socket.read_to_string(&mut response).await.unwrap();
            println!("Response: {}", response);
        }
    };
}
