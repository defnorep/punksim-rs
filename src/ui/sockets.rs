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
    debug!(
        "Socket server listening on: {}",
        listener.local_addr().unwrap()
    );

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        if let Err(e) = handle_connection(peer, stream, rx.clone()).await {
            match e {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => warn!("Error processing connection: {}", err),
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

    info!("New WebSocket connection: {}", peer);

    // this _seems_ like the right place to put this async task.
    tokio::spawn(async move {
        // this is sending as quickly as it receives a new message.
        // we might want to buffer and send every 1 second or so.
        // the fixedtimestep in the bevy app can naturally help buffer the number of
        // messages sent, but ultimately this socket should share in the responsibility
        // of protecting our clients.
        loop {
            let msg = rx.recv_async().await.unwrap();

            match ws_stream.send(Message::Text(msg)).await {
                Ok(_) => {
                    trace!("Message sent to peer {}", peer);
                }
                Err(Error::Io(e)) => {
                    warn!("WebSocket closing: {}, {}", e, peer);
                    break;
                }
                Err(e) => {
                    warn!("Error sending message: {}, {}", e, peer);
                    break;
                }
            }
        }
    });

    Ok(())
}
