use miden::{Assembler, ProgramInputs, ProofOptions, StarkProof};
use miden_core::{Program, ProgramOutputs};
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;
use user::UserData;

// Sanitizes inputs enabling correct serialisation of user_data
fn handle_user_data(
    stark_proof: StarkProof,
    program: Program,
    outputs: ProgramOutputs,
) -> UserData {
    let proof = stark_proof.to_bytes();
    let hash = program
        .hash()
        .as_elements()
        .iter()
        .map(|v| v.inner())
        .collect();
    let outputs_stack = outputs.stack().to_vec();
    let outputs_overflow = outputs.overflow_addrs().to_vec();
    UserData {
        proof,
        hash,
        outputs_stack,
        outputs_overflow,
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Set Network address
    let addr = "127.0.0.1:3000".to_string();

    // Connect to Network
    let mut socket = TcpStream::connect(&addr).await?;

    // instantiate the assembler
    let assembler = Assembler::default();

    // Compile program
    let program = assembler.compile("begin push.3 push.5 add end").unwrap();

    // let's execute it and generate a STARK proof
    let (outputs, stark_proof) =
        miden::prove(&program, &ProgramInputs::none(), &ProofOptions::default()).unwrap();

    // Create user_data struct
    let user_data = handle_user_data(stark_proof, program, outputs);

    // Serialises the user_data for TCP transport
    let serialised = serde_json::to_vec(&user_data).expect("Failed to serialise user_data.");

    // Sending necessary user_data through TCP for proof verification
    // program hash + proof + outputs
    socket.write_all(&serialised).await?;
    println!("Proof sent from User to the Network.");

    Ok(())
}
