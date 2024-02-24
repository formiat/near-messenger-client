use anyhow::{anyhow, Result};
use near_jsonrpc_client::methods::query::RpcQueryRequest;
use near_jsonrpc_client::JsonRpcClient;
use near_jsonrpc_primitives::types::query::{QueryResponseKind, RpcQueryResponse};
use near_primitives::types::BlockReference;
use near_primitives::types::Finality;
use near_primitives::views::QueryRequest;
use serde_json::json;
use tokio::time::{sleep, Duration};

use crate::aes;
use crate::db;
use crate::encrypted_msg::EncryptedMsgWrapper;
use crate::CONTRACT_ACCOUNT_ID;

const READ_MSGS_AT_ONCE_NUM: usize = 10;

pub async fn start_listener(client: JsonRpcClient, aes_key: aes::Key) -> Result<()> {
    loop {
        let last_index_checked = db::get_last_index_checked()?;

        let request = RpcQueryRequest {
            block_reference: BlockReference::Finality(Finality::Final),
            request: QueryRequest::CallFunction {
                account_id: CONTRACT_ACCOUNT_ID.parse().unwrap(),
                method_name: "get_multiple".to_string(),
                args: serde_json::to_vec(
                    &json!({"start_index": last_index_checked, "num_msgs": READ_MSGS_AT_ONCE_NUM}),
                )
                .unwrap()
                .into(),
            },
        };

        match client.call(request).await {
            Ok(response) => {
                if let Err(e) = process_response(response, &aes_key) {
                    println!("Error processing response: {:?}", e);

                    // Wait for extra 5 seconds before the next check
                    sleep(Duration::from_secs(5)).await;
                }
            }
            Err(e) => {
                println!("Error making request: {:?}", e);

                // Wait for extra 5 seconds before the next check
                sleep(Duration::from_secs(5)).await;
            }
        }

        // Wait for 5 seconds before the next check
        sleep(Duration::from_secs(5)).await;
    }
}

fn process_response(response: RpcQueryResponse, aes_key: &aes::Key) -> Result<()> {
    let mut last_index_checked = db::get_last_index_checked()?;

    if let QueryResponseKind::CallResult(call_result) = response.kind {
        let msgs = serde_json::from_slice::<serde_json::Value>(&call_result.result)?;
        let msgs = serde_json::from_value::<Vec<EncryptedMsgWrapper>>(msgs)?;

        if !msgs.is_empty() {
            println!("msgs raw: {:?}", msgs);
            last_index_checked += msgs.len();
            db::set_last_index_checked(last_index_checked)?;
            println!("Updated `last_index_checked`: {}", last_index_checked);

            for msg in msgs {
                println!("Deserialized msg wrapper: {:?}", msg);

                match msg.decrypt_str(aes_key) {
                    Ok(msg) => println!("Deserialized msg: {:?}", msg),
                    Err(e) => println!("Error decrypting msg: {:?}", e),
                }
            }
        } else {
            println!("No new msgs.");
        }
    } else {
        Err(anyhow!("Failed to parse the response: Unexpected result: Expected `QueryResponseKind::CallResult`."))?;
    }

    Ok(())
}

fn extract_vec_from_value(value: serde_json::Value) -> Option<Vec<serde_json::Value>> {
    match value {
        serde_json::Value::Array(vec) => Some(vec),
        _ => None,
    }
}
