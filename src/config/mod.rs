
use std::env;
use std::process;

pub fn get_config() -> Config {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Please supply an argument: 'values' or 'nullvotes'.");
        process::exit(1);
    });
    config
}

pub struct Config {
    pub gossip_type: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments supplied.")
        }
        let gossip_type = args[1].clone();
        match gossip_type.as_ref() {
            "values" | "nullvotes" => Ok(Config { gossip_type }),
            _ => Err("Argument must be 'values' or 'nullvotes'"),
        }
        // match &gossip_type[..]
        // match &gossip_type as &str
        // match gossip_type.as_ref()
    }

}
