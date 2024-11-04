use std::{collections::HashMap, env, process::exit, str::FromStr};
use config::Config;
use outbound::db_driver::DbDriver;

mod inbound;
mod model;
mod outbound;
mod service;

fn main() {
    // read configuration
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        log::error!("application requires first argument to be path of configuration file");
        exit(101);
    }
    let config_path = &args[1];
    
    let config_data = match Config::builder()
        .add_source(config::File::with_name(config_path))
        .build() {
            Ok(v) => v,
            Err(e) => panic!("config error: {}", e),
        };

    let config = match config_data.try_deserialize::<HashMap<String, HashMap<String, String>>>() {
        Ok(v) => v,
        Err(e) => panic!("config error: {}", e)
    };

    // setting the logger
    let log_level = log::LevelFilter::from_str(config["log"]["level"].as_str()).unwrap();
    env_logger::builder()
        .filter_level(log_level)
        .format_target(false)
        .init();

    // starting the application
    log::info!("application [{}] starting...", config["meta"]["id"]);

    let db_client = DbDriver::new(&config);

    inbound::http_router::start(&config, db_client);

}

#[cfg(test)]
mod tests {
    #[test]
    fn try_test() {
        assert_eq!(true, true);
    }
}
