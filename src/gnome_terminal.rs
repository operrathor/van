use std::collections::HashMap;
use std::error::Error;
use std::process::Command;
use std::str;

use crate::Terminal;
use crate::themes::Theme;

pub struct GnomeTerminal;

impl Terminal for GnomeTerminal {
    fn apply(theme: &Theme) -> Result<(), Box<dyn Error>> {
        let settings = Settings::from(theme);
        let profile =
            get_profile().map_err(|_| "Could net determine gnome-terminal's default profile")?;
        apply_settings(settings, &profile).map_err(|_| "Could not apply theme")?;
        Ok(())
    }
}

struct Settings(HashMap<&'static str, String>);

impl IntoIterator for Settings {
    type Item = (&'static str, String);
    type IntoIter = std::collections::hash_map::IntoIter<&'static str, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<&Theme> for Settings {
    fn from(theme: &Theme) -> Self {
        let mut map = HashMap::new();
        let palette_colors: Vec<String> =
            theme.palette.iter().map(|c| format!("'{}'", c)).collect();
        map.insert("palette", format!("[{}]", palette_colors.join(", ")));
        map.insert("foreground-color", theme.foreground.clone());
        map.insert("background-color", theme.background.clone());
        map.insert("use-theme-colors", String::from("false"));
        Settings(map)
    }
}

fn get_profile() -> Result<String, Box<dyn Error>> {
    let output = Command::new("gsettings")
        .arg("get")
        .arg("org.gnome.Terminal.ProfilesList")
        .arg("default")
        .output()?;
    if !output.status.success() {
        return Err("Non-zero exit status")?;
    }
    let profile = str::from_utf8(&output.stdout)?.trim().replace('\'', "");
    Ok(profile)
}

fn apply_settings(settings: Settings, profile: &str) -> Result<(), Box<dyn Error>> {
    let path = format!(
        "org.gnome.Terminal.Legacy.Profile:/org/gnome/terminal/legacy/profiles:/:{}/",
        profile
    );
    for (key, value) in settings {
        let output = Command::new("gsettings")
            .arg("set")
            .arg(&path)
            .arg(key)
            .arg(value)
            .output()?;
        if !output.status.success() {
            return Err("Non-zero exit status")?;
        }
    }
    Ok(())
}
