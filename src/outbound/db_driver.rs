use std::{collections::HashMap, error::Error, time::Duration};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};


#[derive(Clone)]
pub struct DbDriver {
    uri: String,
}

impl DbDriver {
    pub fn new(config: &HashMap<String, HashMap<String, String>>) -> Self {
        let url_string = config["database"]["service_uri"].clone();
        return Self{uri: url_string};
    }

    pub async fn get_db(&self) -> Result<DatabaseConnection, Box<dyn Error>> {
        let mut opt = ConnectOptions::new(self.uri.clone());
        opt.sqlx_logging(true)
            .max_connections(100)
            .connect_timeout(Duration::from_secs(10))
            .sqlx_logging_level(log::LevelFilter::Debug);

        match Database::connect(opt).await {
            Ok(d) => {
                assert!(d.ping().await.is_ok());
                return Ok(d)
            },
            Err(e) => return Err(e.into()),
        };
    }
} 