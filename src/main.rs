mod config;

use config::Config;

fn main() {
    let config = Config::get_input_config();
    println!("query: {} | file_path: {}", config.query, config.file_path);
}
