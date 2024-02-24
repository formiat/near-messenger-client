use anyhow::Result;
use clap::Parser;

use crate::aes;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub enum Command {
    Send {
        #[clap(long)]
        contract_account_id: String,
        #[clap(long)]
        aes_key_file_path: String,
        #[clap(long)]
        recipient_account_id: String,
        #[clap(long)]
        msg: String,
    },
    Receive {
        #[clap(long)]
        contract_account_id: String,
        #[clap(long)]
        aes_key_file_path: String,
    },
}

impl Command {
    pub fn get_aes_key_file_path(&self) -> &str {
        match self {
            Command::Send {
                aes_key_file_path, ..
            }
            | Command::Receive {
                aes_key_file_path, ..
            } => aes_key_file_path,
        }
    }

    pub fn get_aes_key(&self) -> Result<aes::Key> {
        let path = self.get_aes_key_file_path();

        aes::Key::load_from_file(path)
    }
}
