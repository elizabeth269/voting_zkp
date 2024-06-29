# Voting System with Zero-Knowledge Proof

This project implements a simple voting system using Rust, incorporating basic cryptographic functions and a placeholder for zero-knowledge proofs (ZKP).

## Features

- **Voter Registration**: Voters can register with a public key and receive a private key.
- **Vote Casting**: Voters can cast their votes, which are encrypted using their private key.
- **Vote Verification**: The system verifies votes using the corresponding public key.

## Project Structure

- `src/main.rs`: Entry point of the application.
- `src/voter.rs`: Contains the Voter struct and related functions.
- `src/election.rs`: Contains the Election struct and related functions.
- `src/zkproof.rs`: Contains the ZeroKnowledgeProof struct and related functions.

## Dependencies

- `rsa`: For RSA key generation and encryption.
- `rand`: For random number generation.
- `serde` and `serde_json`: For serialization and deserialization of data.

## Usage

1. Clone the repository.
   ```sh
   git clone https://github.com/yourusername/voting_system.git
   cd voting_system
   ```
2. Build the project.
   ```sh
   cargo build
   ```
3. Run the application.
   ```sh
   cargo run
   ```

## Example

The example in `src/main.rs` demonstrates:

1. Creating an election instance.
2. Registering voters.
3. Casting votes.
4. Verifying votes.

## Running Tests

To run the tests, use the following command:

```sh
cargo test
```
