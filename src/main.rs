use std::env;

mod papechange;
fn main() {
    //let args: Vec<String> = env::args().collect();
    #[cfg(target_os = "linux")]
    match papechange::pape_amnesia(
        "/home/quinton/Pictures/img/papes/Anime/Code Geass/1686815339034879.jpg",
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
