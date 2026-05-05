use std::env;

mod papechange;
fn main() {
    //let args: Vec<String> = env::args().collect();
    #[cfg(target_os = "linux")]
    match papechange::pape_amnesia(
        "/home/quinton/Pictures/img/papes/Brutalist/1739137795224378.jpg",
    ) {
        Ok(val) => {
            println!("Val {:?}", val)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    //match on args
}
