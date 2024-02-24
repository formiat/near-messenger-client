use anyhow::Result;
use clap::Parser;
use near_jsonrpc_client::JsonRpcClient;

use crate::command::Command;
use crate::listener::start_listener;

mod aes;
mod command;
mod db;
mod encrypted_msg;
mod listener;

pub const CONTRACT_ACCOUNT_ID: &str = "formiat4.testnet";

#[tokio::main]
async fn main() -> Result<()> {
    let command = Command::parse();
    // panic!("command: {command:?}");

    let client = JsonRpcClient::connect("https://rpc.testnet.near.org");

    match command {
        Command::Send { .. } => todo!(),
        Command::Receive { .. } => start_listener(client, command.get_aes_key()?).await?,
    }

    Ok(())
}
