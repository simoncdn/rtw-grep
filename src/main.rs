use rtw_grep::config::Config;
use std::process;

fn main() {
    let config = Config::get_input_config();
    if let Err(e) = rtw_grep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    };
}
