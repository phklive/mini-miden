use tokio::io::{self};
use tokio::net::{TcpListener, TcpStream};

// Configure listening address
const LISTENER_ADDR: &str = "127.0.0.1:3000";

// Configure forwarding address
const FORWARDER_ADDR: &str = "127.0.0.1:3001";

#[tokio::main]
async fn main() -> io::Result<()> {
    // Listen to messages sent from User
    let listener = TcpListener::bind(LISTENER_ADDR).await?;
    println!("Network running on {LISTENER_ADDR}.");

    // Loop to keep scanning for outbound connections and TCPstreams
    loop {
        let (mut socket, _) = listener.accept().await?;

        // Connect to verifier
        let mut forward = TcpStream::connect(FORWARDER_ADDR).await?;

        tokio::spawn(async move {
            // Forward TCP stream from User to Verifier
            // Mocking Aggregation and Recursive prooving step
            match tokio::io::copy(&mut socket, &mut forward).await {
                Ok(_) => {
                    println!("Proof received from User, processing.");
                    println!("Network aggregated and created recursive proof.");
                    println!("Proof sent to Verifier.");
                }
                Err(e) => {
                    eprintln!("Failed to forward data: {}", e);
                }
            }
        });
    }
}
