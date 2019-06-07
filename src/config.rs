use std::fmt;

pub struct Config {
    addr: &'static str, //服务器地址
    local_sign: bool,   //本地签名
}

impl Config {
    pub fn new(addr: &'static str, local_sign: bool) -> Box<Self> {
        Box::new( Config {
            addr: addr,
            local_sign: local_sign,
        })
    }
}

//实现fmt方法，或者使用#[derive(Debug)]
impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Config {{ addr: {}, local_sign: {} }}", self.addr, self.local_sign)
    }
}