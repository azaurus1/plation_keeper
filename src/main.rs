extern crate dotenv;
extern crate tokio;

use std::str::FromStr;
use std::time;
use hex_literal::hex;
use dotenv::dotenv;
use web3::{Web3};
use futures::{self, StreamExt};

#[tokio::main]
async fn main() -> web3::Result {
    dotenv().ok();

    //Env variables
    let private_key = "PRIV_KEY";
    let private_key = dotenv::var(private_key).unwrap();

    let rpc_key = "RPC";
    let rpc = dotenv::var(rpc_key).unwrap();

    let prediction_handler_address = web3::types::Address::from_str("0x122dF77D0069667EFA788aD78650630cb7e8170d").unwrap();
    let genesis_block = web3::types::U64::from_str("10735044").unwrap();
    let prediction_handler_genesis_block = web3::types::BlockNumber::Number(genesis_block);
    let inflation_oracle_address = web3::types::Address::from_str("0xCa4B1B05AA433Fc397959cEDdb897DBAFe9C8E87").unwrap();


    //Web3 inits
    let transport = web3::transports::Http::new(&rpc)?;
    let web3 = web3::Web3::new(transport);
    let address = web3::types::Address::from_str("0x0a29bd68d085CcFa30bDFf2c2ca849D5976a4C9D").unwrap();
    let latest = Some(web3::types::BlockNumber::Latest);
    let nonce = web3.eth().transaction_count(address, latest).await?;
    let topic = web3::types::H256::from_str("0xd07645f01fc3b41f48d3d8c7f791363e20ddd80557a6d92f734a2ee160cbd9f8").unwrap();

    //Contract inits
    let prediction_handler_contract = web3::contract::Contract::from_json(web3.eth(),prediction_handler_address,include_bytes!("./contracts/PredictionHandler.abi")).unwrap();
    let inflation_oracle_contract =  web3::contract::Contract::from_json(web3.eth(),inflation_oracle_address,include_bytes!("./contracts/InflationFeed.abi")).unwrap();
    
    //Event filters
    let created_filter = web3::types::FilterBuilder::default()
        .address(vec![prediction_handler_address])
        .from_block(prediction_handler_genesis_block)
        .topics(
                Some(vec![topic]),
                None,
                None,
                None,
            )
                .build();

    //println!("{:?}",prediction_handler_contract);
    //println!("{:?}",inflation_oracle_contract);

    let filter = web3.eth_filter().create_logs_filter(created_filter).await?;

    let logs_stream = filter.stream(time::Duration::from_secs(1));
    futures::pin_mut!(logs_stream);
    
    //let log = logs_stream.next().await.unwrap().unwrap();
    //println!("got log: {:?}", log);
    

    Ok(())

}
