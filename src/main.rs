use std::env;


mod papechange;
fn main() {
    //let args: Vec<String> = env::args().collect();
    #[cfg(target_os = "linux")]
    papechange::changepape();
    //match on args
}
