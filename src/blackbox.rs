use std::error::Error;
use std::fs;
use std::path::Path;

use configparser::ini::Ini;
use serde::Serialize;

use crate::Terminal;
use crate::themes::Theme;

pub struct BlackBox;

impl Terminal for BlackBox {
    fn apply(theme: &Theme) -> Result<(), Box<dyn Error>> {
        let home_dir = dirs::home_dir().ok_or("")?;
        if home_dir.join(".var/app/com.raggesilver.BlackBox").is_dir() {
            write_and_update(theme, &home_dir.join(".var/app/com.raggesilver.BlackBox"))?;
        }
        if home_dir.join(".var/app/com.raggesilver.BlackBox.Devel").is_dir() {
            write_and_update(theme, &home_dir.join(".var/app/com.raggesilver.BlackBox.Devel"))?;
        }
        Ok(())
    }
}

fn write_and_update(theme: &Theme, dir: &Path) -> Result<(), Box<dyn Error>> {
    write(theme, dir)?;
    update(theme, dir)?;
    Ok(())
}

fn update(theme: &Theme, dir: &Path) -> Result<(), Box<dyn Error>> {
    let settings_dir = dir.join("config/glib-2.0/settings");
    fs::create_dir_all(&settings_dir)?;
    let keyfile = settings_dir.join("keyfile");
    let mut ini = Ini::new_cs();
    if keyfile.is_file() {
        ini.load(&keyfile)?;
    }
    ini.set("com/raggesilver/BlackBox", "theme-dark", Some(format!("'{}'", theme.name)));
    ini.write(&keyfile)?;
    Ok(())
}

fn write(theme: &Theme, dir: &Path) -> Result<(), Box<dyn Error>> {
    let scheme = Scheme {
        name: theme.name.clone(),
        use_theme_colors: false,
        foreground_color: theme.foreground.clone(),
        background_color: theme.background.clone(),
        palette: theme.palette.clone(),
    };
    let schemes_dir = dir.join("data/blackbox/schemes");
    fs::create_dir_all(&schemes_dir)?;
    let path = schemes_dir.join(format!("{}.json", theme.name.to_lowercase()));
    let writer = &fs::File::create(path.as_path())?;
    serde_json::to_writer_pretty(writer, &scheme)?;
    Ok(())
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct Scheme {
    pub name: String,
    pub use_theme_colors: bool,
    pub foreground_color: String,
    pub background_color: String,
    pub palette: [String; 16],
}
