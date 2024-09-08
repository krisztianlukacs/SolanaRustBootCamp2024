use solana_client::rpc_client::RpcClient;

fn main() {
    // Connect to the Devnet RPC endpoint
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");

    // Load the keypair from the file
    let keypair_path = "~/my-solana-wallet/my-keypair.json"; // Replace with the actual path to your keypair file
    let keypair = solana_sdk::signer::keypair::read_keypair_file(keypair_path)
        .expect("Failed to read keypair file");

    // Get the balance of the account associated with the keypair
    let balance = rpc_client.get_balance(&keypair.pubkey())
        .expect("Failed to get balance");

    println!("Balance: {} SOL", balance);
}