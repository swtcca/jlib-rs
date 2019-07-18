
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::message::query::server_info::ServerInfoCommand;
use crate::message::query::account_tx::RequestAccountTxCommand;
use crate::message::query::account_info::RequestAccountInfoCommand;

#[derive(Debug)]
struct SuperError {
    side: &'static dyn Error,
}

impl fmt::Display for SuperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl Error for SuperError {
    fn description(&self) -> &str {
        self.side.description()
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.side)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfoSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : ServerInfoCommand,
    pub status          : String,
    pub rtype           : String,
}

impl fmt::Display for ServerInfoSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ServerInfoSideKick is here!")
    }
}

impl Error for ServerInfoSideKick  {
    fn description(&self) -> &str {
        "I'm ServerInfoSideKick side kick"
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfoSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : RequestAccountInfoCommand,
    pub status          : String,
    pub rtype            : String,
}

impl fmt::Display for AccountInfoSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AccountInfoSideKick is here!")
    }
}

impl Error for AccountInfoSideKick  {
    fn description(&self) -> &str {
        "I'm AccountInfoSideKick side kick"
    }
}

//AccounTx
#[derive(Debug, Serialize, Deserialize)]
pub struct AccounTxSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : RequestAccountTxCommand,
    pub status          : String,
    pub rtype            : String,
}

impl fmt::Display for AccounTxSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AccounTxSideKick is here!")
    }
}

impl Error for AccounTxSideKick  {
    fn description(&self) -> &str {
        "I'm AccounTxSideKick side kick"
    }
}
    // let y: &'static ServerInfoSideKick = &ServerInfoSideKick{};
    // let e = SuperError { side: y };
    // println!("Error: {}", e.description());
    // println!("caused by : {}", e.source().unwrap());



