use std::result::Result;

use iroh::{
    endpoint::Connection,
    protocol::{AcceptError, ProtocolHandler},
};
use tracing::info;

/// The ALPN protocol identifier for the echo service
pub const ECHO_ALPN: &[u8] = b"ECHO";

/// Echo protocol handler
#[derive(Debug)]
pub struct EchoProtocol;

impl ProtocolHandler for EchoProtocol {
    async fn accept(&self, conn: Connection) -> Result<(), AcceptError> {
        info!("Connection accepted");

        // Accept a bi-directional stream
        let (mut send_stream, mut recv_stream) =
            conn.accept_bi().await.map_err(AcceptError::from_err)?;

        // Read the message
        let msg = recv_stream
            .read_to_end(1024)
            .await
            .map_err(AcceptError::from_err)?;
        info!("Received message: {}", String::from_utf8_lossy(&msg));

        // Echo the message back
        send_stream
            .write_all(&msg)
            .await
            .map_err(AcceptError::from_err)?;
        send_stream.finish()?;

        // Wait for the client to close the connection
        conn.closed().await;
        info!("Connection closed");

        Ok(())
    }
}
