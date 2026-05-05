use std::env;
#[cfg(target_os = "linux")]
use std::path::PathBuf;

mod papechange;
fn main() {
    //let args: Vec<String> = env::args().collect();
    #[cfg(target_os = "linux")]
    let val: PathBuf =
        "/home/quinton/Pictures/img/papes/Anime/Code Geass/1686815339034879.jpg".into();
    papechange::changepape(&val);
    /*
    match papechange::pape_amnesia(
        ,
    ) {
        Ok(val) => {
            println!("Val {:?}", val)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    } */
    //match on args
}
