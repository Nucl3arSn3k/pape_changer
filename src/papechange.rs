#[cfg(target_os = "linux")]
use dbus::blocking::Connection;
use rand::{RngExt, rng};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::time::Duration;
#[cfg(target_os = "linux")]
pub fn changepape_rand() {
    let entries: Vec<_> = std::fs::read_dir("/home/quinton/Pictures/img/papes") //Mod to dummy out dirs lists them in toplevel. muy no bueno
        .expect("Failed to read directory")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|d| d.is_file())
        .collect();

    if entries.is_empty() {
        return;
    }

    let idx = rng().random_range(0..entries.len());
    let pape = &entries[idx];
    set_wallpaper(pape.to_str().unwrap());
    println!("{}", pape.display());
}

pub fn changepape(papepath: &PathBuf) {
    let pape = papepath;

    set_wallpaper(pape.to_str().unwrap());
}

#[cfg(target_os = "linux")]
fn set_wallpaper(path: &str) {
    //Set and remove somehow
    let conn = Connection::new_session().expect("Failed to connect to DBus");
    let proxy = conn.with_proxy(
        "org.kde.plasmashell",
        "/PlasmaShell",
        Duration::from_millis(6000),
    );
    //JS for internal scripting,EWWWWW
    let script = format!(
        r#"
        var allDesktops = desktops();
    for (var i = 0; i < allDesktops.length; i++) {{
        var d = allDesktops[i];
        d.wallpaperPlugin = 'org.kde.image';
    d.currentConfigGroup = ['Wallpaper', 'org.kde.image', 'General'];
    d.writeConfig('Image', 'file://{}');
}}
"#,
        path
    );

    proxy
        .method_call::<(), _, _, _>("org.kde.PlasmaShell", "evaluateScript", (script,))
        .expect("Failed to set wallpaper");

    match pape_amnesia(path) {
        Ok(o) => println!("{:?}", o),
        Err(e) => eprintln!("{}", e),
    }
}

//So the wallpaper is kept in ~/.config/plasma-org.kde.plasma.desktop-appletsrc file
//get full pwd for that and then see if I can snipe it out of recents while leaving it set....hrmm
//
#[cfg(target_os = "linux")]
pub fn pape_amnesia(path: &str) -> std::io::Result<()> {
    use std::io::Write;

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/home/quinton/.config/plasmarc")?; //actually here. Match on path probably?
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let new_contents = contents //will not work on first entry,split logic isn't great for now.
        .split(",")
        .filter(|x| x.trim() != path)
        .collect::<Vec<&str>>()
        .join(",");
    println!("{}", new_contents);
    file.seek(std::io::SeekFrom::Start(0))?;
    file.set_len(0)?;
    file.write_all(new_contents.as_bytes())?; //into bytes heavier,so as_bytes better here

    Ok(())
}
