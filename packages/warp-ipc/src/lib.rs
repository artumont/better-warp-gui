use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use tokio::net::UnixStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

pub mod protocol;

static SOCKET_PATH: &str = "/run/cloudflare-warp/warp_service";

/// Asynchronous IPC client for communicating with the Cloudflare Warp service.
pub struct WarpIpcClient {
    framed_stream: Framed<UnixStream, LengthDelimitedCodec>,
}

impl WarpIpcClient {
    /// Creates a new instance of the asynchronous IPC client and connects to the Warp service.
    pub async fn new() -> std::io::Result<Self> {
        let stream = UnixStream::connect(SOCKET_PATH).await?;
        let framed_stream = Framed::new(stream, LengthDelimitedCodec::new());
        Ok(Self { framed_stream })
    }

    /// Sends a request to the Warp service and waits for the response.
    pub async fn send_request(
        &mut self,
        request: protocol::Request,
    ) -> std::io::Result<protocol::Response> {
        let request_bytes = serde_json::to_vec(&request)?;
        self.framed_stream.send(request_bytes.into()).await?;

        if let Some(response_bytes) = self.framed_stream.next().await {
            let response_bytes = response_bytes?;
            let response: protocol::Response = serde_json::from_slice(&response_bytes)?;
            Ok(response)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Connection closed by server",
            ))
        }
    }
}
