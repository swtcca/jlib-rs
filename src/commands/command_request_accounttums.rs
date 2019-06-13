#![allow(unused)]

use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::any::Any;

use crate::commands::command_trait::CommandConversion;

/*
@4.9 获得账号可接收和发送的货币
RequestAccountTumsCommand 请求格式
id: u64, //(固定值): 1
command: String, //(固定值): account_currencies
relation_type: Option<String>, //None
account: String, //需要用户传递的参数，钱包的地址
ledger_index: String //(固定值): 'validated'
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountTumsCommand {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    command: String,

    #[serde(rename="relation_type")]
    relation_type: Option<String>,

    #[serde(rename="account")]
    account: String,

    #[serde(rename="ledger_index")]
    ledger_index: String,
}

impl RequestAccountTumsCommand {
    pub fn with_params(account: String) -> Box<Self> {
        Box::new( 
            RequestAccountTumsCommand {
                id: 1,
                command: "account_currencies".to_string(),
                relation_type: None,
                account: account,
                ledger_index: "validated".to_string(),
            }
        )
    }
}

impl CommandConversion for RequestAccountTumsCommand {
    type T = RequestAccountTumsCommand;
    fn to_string(&self) -> Result<String> {
        // let json = json!({ "id": "0", "command": "subscribe" , "streams" : ["ledger","server","transactions"]});
        // let compact = format!("{}", json);

        //https://crates.io/crates/serde_json
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&self)?;

        // Print, write to a file, or send to an HTTP server.
        println!("{}", j);

        Ok(j)
    }
    
    fn box_to_raw(&self) -> &dyn Any {
        self
    }

    // fn to_concrete<T>(&self) -> T {
    //     let def: Box<dyn CommandConversion> = self;
    //     let b: &SubscribeCommand = match def.box_to_raw().downcast_ref::<SubscribeCommand>() {
    //         Some(b) => b,
    //         None => panic!("&a isn't a B!"),
    //     };
        
    //     b
    // }
}

//实现default方法, 此command不提供default方法~
// impl Default for RequestLedgerCommand {
//     fn default() -> Self {
//         RequestLedgerCommand { 
//             id: 1,
//             command: "ledger".to_string(),
//         }
//     }
// }

/////////////////////////
/*
RequestAccountInfoResponse 数据返回格式
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountTumsResponse {
    #[serde(rename="ledger_hash")]
    pub ledger_hash: String,

    #[serde(rename="ledger_index")]
    pub ledger_index: u64,

    #[serde(rename="receive_currencies")]
    pub receive_currencies: Vec<String>,

    #[serde(rename="send_currencies")]
    pub send_currencies: Vec<String>,

    #[serde(rename="validated")]
    pub validated: bool,
}