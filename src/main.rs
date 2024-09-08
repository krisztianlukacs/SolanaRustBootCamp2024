use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Signer;

// Reading SOL Wallet Balance, but Balance: 1000000000 SOL, 1 SOL = 1000000000 lamports

fn main() {
    // Connect to the Devnet RPC endpoint
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");

    // Load the keypair from the file
    let keypair_path = "/home/vscode/my-keypair2.json"; 
    let keypair = solana_sdk::signer::keypair::read_keypair_file(keypair_path)
        .expect("Failed to read keypair file");

    // Get the balance of the account associated with the keypair
    let balance = rpc_client.get_balance(&keypair.pubkey())
        .expect("Failed to get balance");

    println!("Balance: {} lamport", balance);
    println!("Balance: {} SOL", balance as f64 / 1000000000.0);
}