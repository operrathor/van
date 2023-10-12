use std::error::Error;

use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;

const THEMES_URL: &str = "https://raw.githubusercontent.com/Gogh-Co/Gogh/master/data/themes.json";

fn hex_parser(s: &str) -> Result<String, String> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^#[A-Fa-f0-9]{6}$").unwrap();
    }
    if REGEX.is_match(s) {
        Ok(String::from(s))
    } else {
        Err(String::from("Invalid format"))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Root {
    themes: Vec<RawTheme>,
}

macro_rules! structs_for_colors {
    ($($color:ident),+) => {
        #[derive(Parser)]
        #[clap(version)]
        pub struct Args {
            #[clap(value_parser, index = 1)]
            pub theme_name: String,
            $(
                #[clap(long, value_parser = hex_parser)]
                pub $color: Option<String>,
            )+
            #[clap(short, long, value_parser = hex_parser)]
            pub foreground: Option<String>,
            #[clap(short, long, value_parser = hex_parser)]
            pub background: Option<String>
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        struct RawTheme {
            name: String,
            $(
                $color: String,
            )+
            foreground: String,
            background: String
        }

        fn palette_colors<'a>(args: &'a Args, theme: &'a RawTheme) -> [String; 16] {
            [$(
                String::from(args.$color.as_ref().unwrap_or(&theme.$color))
            ,)+]
        }
    }
}

structs_for_colors!(
    color_01,
    color_02,
    color_03,
    color_04,
    color_05,
    color_06,
    color_07,
    color_08,
    color_09,
    color_10,
    color_11,
    color_12,
    color_13,
    color_14,
    color_15,
    color_16
);

pub struct Theme {
    pub name: String,
    pub palette: [String; 16],
    pub foreground: String,
    pub background: String,
}

fn fetch_themes() -> Result<Root, Box<dyn Error>> {
    let root = reqwest::blocking::get(THEMES_URL)?.json::<Root>()?;
    Ok(root)
}

fn find_theme<'r>(name: &str, root: &'r Root) -> Option<&'r RawTheme> {
    root.themes
        .iter()
        .find(|t| t.name.to_lowercase().eq(&name.to_lowercase()))
}

fn merge(theme: &RawTheme, args: &Args) -> Theme {
    Theme {
        name: theme.name.clone(),
        palette: palette_colors(args, theme),
        foreground: args
            .foreground
            .as_ref()
            .map_or(theme.foreground.clone(), String::from),
        background: args
            .background
            .as_ref()
            .map_or(theme.background.clone(), String::from),
    }
}

pub fn get(args: &Args) -> Result<Theme, Box<dyn Error>> {
    let themes = fetch_themes().map_err(|_| "Could not fetch themes")?;
    let theme = find_theme(&args.theme_name, &themes).ok_or("Could not find theme")?;
    Ok(merge(theme, args))
}
