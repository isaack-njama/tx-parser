extern crate bitcoincore_rpc;

use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::env;

fn main() {
    let rpc_user = env::var("RPC_USER").expect("RPC_USER not found in environment");
    let rpc_password = env::var("RPC_PASSWORD").expect("RPC_PASSWORD not found in environment");

    let rpc = Client::new("http://localhost:18332",
                          Auth::UserPass(rpc_user.to_string(), rpc_password.to_string()))
        .expect("Failed to create RPC client");

    let wallet_name = String::from("isaack-wallet");
    let disabled_private_keys = Some(false);
    let blank = Some(false);
    let passphrase = Some("isaack");
    let avoid_reuse = Some(true);

    let btc_wallet = rpc.create_wallet(&wallet_name, disabled_private_keys, blank, passphrase, avoid_reuse);

    match btc_wallet {
        Ok(wallet_identifier) => {
            println!("Wallet created. Identifier: {:?}", wallet_identifier.name);

            if let Err(e) = rpc.load_wallet(&wallet_identifier.name) {
                println!("Error loading wallet: {}", e);
                return;
            }

            if let Ok(wallet_info) = rpc.get_wallet_info() {
                println!("Wallet name: {:?}", wallet_info.wallet_name);
            } else {
                println!("Failed to get wallet info");
            }

            // Decode transaction example
            let tx_hex = "020000000001010ccc140e766b5dbc884ea2d780c5e91e4eb77597ae64288a42575228b79e234900000000000000000002bd37060000000000225120245091249f4f29d30820e5f36e1e5d477dc3386144220bd6f35839e94de4b9cae81c00000000000016001416d31d7632aa17b3b316b813c0a3177f5b6150200140838a1f0f1ee607b54abf0a3f55792f6f8d09c3eb7a9fa46cd4976f2137ca2e3f4a901e314e1b827c3332d7e1865ffe1d7ff5f5d7576a9000f354487a09de44cd00000000";
            let is_witness = Some(true);

            if let Ok(decoded_tx) = rpc.decode_raw_transaction(tx_hex, is_witness) {
                println!("Decoded Tx: {:?}", decoded_tx);
            } else {
                println!("Failed to decode transaction");
            }
        }
        Err(e) => println!("Failed to create wallet: {}", e),
    }
}
