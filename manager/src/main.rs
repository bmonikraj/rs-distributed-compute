use std::{collections::HashMap, env, process::exit};
use config::Config;

mod inbound;
mod model;
mod outbound;
mod service;
mod util;

fn main() {
    // setting the logger
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        .init();

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

    // starting the application
    log::info!("manager [{}] starting...", config["meta"]["id"]);

    inbound::http_router::start(&config);
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn try_test() {
        assert_eq!(true, true);
    }
}
