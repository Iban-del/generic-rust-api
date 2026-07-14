use generic_tool::get_ip;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AddrConfig {
    pub port: u16,
    pub addr: Option<[u8; 4]>,
}

impl AddrConfig {
    pub fn format(&self) -> Result<String, local_ip_address::Error> {
        let addr_list: String = match self.addr {
            Some(ad) => format!("{}.{}.{}.{}", ad[0], ad[1], ad[2], ad[3]),
            None => get_ip()?,
        };

        Ok(format!("{}:{}", addr_list, self.port))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub name: String,
    pub addr: AddrConfig,
}
