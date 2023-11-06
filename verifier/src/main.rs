use miden::StarkProof;
use miden_core::{Felt, ProgramOutputs};
use tokio::io::{self, AsyncReadExt};
use tokio::net::TcpListener;
use user::UserData;

// Configure local address
const ADDR: &str = "127.0.0.1:3001";

// Sanitize parameters enabling correct usage of received user_data before proof verification
fn handle_user_data(user_data: UserData) -> (StarkProof, [Felt; 4], ProgramOutputs) {
    let outputs_stack: Vec<Felt> = user_data.outputs_stack.into_iter().map(Felt::new).collect();
    let outputs_overflow: Vec<Felt> = user_data
        .outputs_overflow
        .into_iter()
        .map(Felt::new)
        .collect();

    let proof =
        StarkProof::from_bytes(&user_data.proof).expect("Failed to deserialise Stark proof.");
    let hash: [Felt; 4] = user_data
        .hash
        .into_iter()
        .map(Felt::from_mont)
        .collect::<Vec<Felt>>()
        .try_into()
        .expect("Failed to convert u64 hash to Felt.");
    let outputs = ProgramOutputs::from_elements(outputs_stack, outputs_overflow);

    (proof, hash, outputs)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Listen to messages sent from network
    let listener = TcpListener::bind(ADDR).await?;
    println!("Verifier running on {ADDR}.");

    // Loop to keep scanning for outbound connections and TCPstreams
    loop {
        // Accept tcp connections
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![];

            // Reading data from TCP socket
            if let Err(e) = socket.read_to_end(&mut buf).await {
                eprint!("Failed to read from socket: {e}")
            }

            // Deserialisation of the user_data
            let user_data: UserData =
                serde_json::from_slice(&buf).expect("Failed to deserialise UserData struct.");

            // Handling of the proof, hash and outputs
            let (proof, hash, outputs) = handle_user_data(user_data);

            // Using miden::verify() run the miden verifier to validate the proof correctness
            match miden::verify(hash.into(), &[], &outputs, proof) {
                Ok(_) => println!("Proof has been verified!"),
                Err(msg) => println!("There has been an error: {}", msg),
            }
        });
    }
}
