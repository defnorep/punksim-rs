use flume::Receiver;
use futures_util::SinkExt;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Result},
};
use tungstenite::Message;

pub async fn socket_startup(rx: Receiver<String>) {
    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        println!("Peer address: {}", peer);

        if let Err(e) = handle_connection(peer, stream, rx.clone()).await {
            match e {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => println!("Error processing connection: {}", err),
            }
        }
    }
}

async fn handle_connection(
    peer: SocketAddr,
    stream: TcpStream,
    rx: Receiver<String>,
) -> Result<()> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    println!("New WebSocket connection: {}", peer);

    // this _seems_ like the right place to put this async task.
    tokio::spawn(async move {
        // this is sending as quickly as it receives a new message.
        // we might want to buffer and send every 1 second or so.
        // the fixedtimestep in the bevy app can naturally help buffer the number of
        // messages sent, but ultimately this socket should share in the responsibility
        // of protecting our clients.
        loop {
            let msg = rx.recv().unwrap();

            match ws_stream.send(Message::Text(msg)).await {
                Ok(_) => (),
                Err(Error::Io(e)) => {
                    println!("WebSocket closing: {}, {}", e, peer);
                    break;
                }
                Err(e) => {
                    println!("Error sending message: {}, {}", e, peer);
                    break;
                }
            }
        }
    });

    Ok(())
}
