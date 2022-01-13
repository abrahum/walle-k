mod config;

const WALLE_K: &str = "Walle-K";

#[tokio::main]
async fn main() {
    use rs_khl::prelude::{EchoHandler, KHL};
    use walle_core::impls::OneBot;

    let config = config::Config::load_from_file();
    let khl = KHL::new_from_config(config.khl, EchoHandler).arc();
    let ob = OneBot::new(WALLE_K, "kaiheila", "Unkown", config.ob, action_handler);
}
