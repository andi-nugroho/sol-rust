use poem::{
    Route, Server, get, handler,
    listener::TcpListener,
    post,
    web::{Json, Path},
};
use solana_client::rpc_client::RpcClient;
use solana_program::{hash::hash, pubkey::Pubkey, system_instruction::transfer};

use solana_sdk::{
    message::Message,
    native_token::LAMPORTS_PER_SOL,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
};
// const RPC_URL: &str = "http://127.0.0.1:8899";
const RPC_URL: &str = "https://api.devnet.solana.com";
use serde::{Deserialize, Serialize};
use std::{str::FromStr, time::Duration};
use std::thread;

#[derive(Serialize, Deserialize)]
pub struct GetBalance {
    pub wallet: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetAirdrop {
    pub wallet: String,
    pub sol: u64,
}

#[derive(Serialize, Deserialize)]
pub struct GetBalanceRespose {
    pub wallet_add: String,
    pub balance_lamports: u64,
    pub balance_sol: f64,
}

#[derive(Serialize, Deserialize)]
pub struct GetAirdropRespose {
    pub wallet_add: String,
    pub previous_balance_lamports: u64,
    pub new_balance_lamports: u64,
    pub new_balance_sol: f64,
}

#[handler]
fn get_balance(Json(data): Json<GetBalance>) -> Json<GetBalanceRespose> {
    let wallet_add = data.wallet;
    let client = RpcClient::new(RPC_URL);
    let wallet = wallet_add.parse::<Pubkey>().unwrap();
    let balance = client.get_balance(&wallet).unwrap(); 
    Json(GetBalanceRespose {
        wallet_add,
        balance_lamports: balance,
        balance_sol: balance as f64 / 1_000_000_000.0,
    })
}

#[handler]
fn get_airdrop(Json(data): Json<GetAirdrop>) -> Json<GetAirdropRespose> {
    let wallet_add = data.wallet;
    let airdro_amount = data.sol;
    let client = RpcClient::new(RPC_URL);
    let wallet = wallet_add.parse::<Pubkey>().unwrap();
    let old_balance = client.get_balance(&wallet).unwrap();

    match client.request_airdrop(&wallet, airdro_amount * LAMPORTS_PER_SOL) {
        Ok(s) => {
            println!("Success! Check out your TX here:");
            println!(
                "https://explorer.solana.com/tx/{}?cluster=custom",
                s.to_string()
            );
            client.confirm_transaction(&s).expect("Tx confirmation failed");

        }
        Err(e) => println!("Opps , Kuchh to gadbad hai re baba: {}", e.to_string()),
    };

    // thread::sleep(Duration::from_secs(10));

    let new_balance = client.get_balance(&wallet).unwrap();

    Json(GetAirdropRespose {
        wallet_add,
        previous_balance_lamports: old_balance,
        new_balance_lamports: new_balance,
        new_balance_sol: new_balance as f64 / 1_000_000_000.0,
    })
}

#[handler]
fn landing_page() -> String {
    String::from("Hello Its me Prince, you can call /get_balance with your wallet PublicKey & also feel free to play with /get_airdrop although there is 69% chance that it will throw limit exhausted")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
         .at("/",get(landing_page))
        .at("/get_balance", get(get_balance))
        .at("/get_airdrop", get(get_airdrop));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
