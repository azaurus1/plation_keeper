extern crate dotenv;
extern crate tokio;

use std::str::FromStr;
use hex_literal::hex;
use dotenv::dotenv;
use web3::{Web3, futures::TryFutureExt};

#[tokio::main]
async fn main() -> web3::Result {
    dotenv().ok();

    //Env variables
    let private_key = "PRIV_KEY";
    let private_key = dotenv::var(private_key).unwrap();

    let rpc_key = "RPC";
    let rpc = dotenv::var(rpc_key).unwrap();

    let prediction_handler_address = web3::types::Address::from_str("0x122dF77D0069667EFA788aD78650630cb7e8170d").unwrap();
    let inflation_oracle_address = web3::types::Address::from_str("0xCa4B1B05AA433Fc397959cEDdb897DBAFe9C8E87").unwrap();


    //Web3 inits
    let transport = web3::transports::Http::new(&rpc)?;
    let web3 = web3::Web3::new(transport);
    let address = web3::types::Address::from_str("0x0a29bd68d085CcFa30bDFf2c2ca849D5976a4C9D").unwrap();
    let latest = Some(web3::types::BlockNumber::Latest);
    let nonce = web3.eth().transaction_count(address, latest).await?;

    //Contract inits
    let prediction_handler_contract = web3::contract::Contract::from_json(web3.eth(),prediction_handler_address,include_bytes!("./contracts/PredictionHandler.abi"));
    let inflation_oracle_contract =  web3::contract::Contract::from_json(web3.eth(),inflation_oracle_address,include_bytes!("./contracts/InflationFeed.abi"));
    
    //Event filters
    println!("{}",inflation_oracle_contract.address());

    Ok(())

}
