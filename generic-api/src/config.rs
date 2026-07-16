/// type de bdd sql gérer
#[derive(Debug, Clone, serde::Deserialize, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum SqlDatabaseType {
    Postgres,
    MySql,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SqlConnector {
    pub db_type: SqlDatabaseType,
    pub url: String,
    pub alias: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Database {
    pub sql_connectors: Vec<SqlConnector>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AddrConfig {
    pub port: u16,
    pub addr: Option<[u8; 4]>,
}

impl AddrConfig {
    pub fn format(&self) -> Result<String, local_ip_address::Error> {
        let addr_list: String = match self.addr {
            Some(ad) => format!("{}.{}.{}.{}", ad[0], ad[1], ad[2], ad[3]),
            None => generic_tool::get_ip()?,
        };

        Ok(format!("{}:{}", addr_list, self.port))
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub name: String,
    pub addr: AddrConfig,
    pub database: Database,
}
