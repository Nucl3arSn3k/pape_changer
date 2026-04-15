use rand::{RngExt, rng};
use std::time::Duration;

#[cfg(target_os = "linux")]
use dbus::blocking::Connection;
#[cfg(target_os = "linux")]
pub fn changepape() {
    let entries: Vec<_> = std::fs::read_dir("/home/quinton/Pictures/img/papes") //Mod to dummy out dirs lists them in toplevel. muy no bueno
    .expect("Failed to read directory")
    .filter_map(|e| e.ok())
    .map(|e| e.path())
    .filter(|d| d.is_file())
    .collect();

    if entries.is_empty() { return; }

    let idx = rng().random_range(0..entries.len());
    let pape = &entries[idx];
    set_wallpaper(pape.to_str().unwrap());
    println!("{}", pape.display());
}


#[cfg(target_os = "linux")]
fn set_wallpaper(path: &str) { //Set and remove somehow
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

    proxy.method_call::<(), _, _, _>("org.kde.PlasmaShell", "evaluateScript", (script,))
    .expect("Failed to set wallpaper");
}

//So the wallpaper is kept in ~/.config/plasma-org.kde.plasma.desktop-appletsrc file
//get full pwd for that and then see if I can snipe it out of recents while leaving it set....hrmm
//

