use crate::utils;
use eframe::egui;
use eframe::egui::FontFamily::{Monospace, Proportional};
use eframe::egui::{FontId, TextStyle};
use rand::Rng;
use rust_embed::RustEmbed;
use std::borrow::Cow;
use std::path::PathBuf;
use std::{fs, io};

#[derive(RustEmbed)]
#[folder = "font"]
struct FontAsset;

fn get_font(font_name: &str) -> Vec<u8> {
    let file = FontAsset::get(font_name).unwrap();
    let cow = file.data;
    cow.to_vec()
}

pub(crate) fn configure_text_styles(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(16.0, Monospace)),
        (TextStyle::Button, FontId::new(16.0, Proportional)),
        (TextStyle::Small, FontId::new(12.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}

pub(crate) fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.

    fonts.font_data.insert(
        "sarasa_gothic".to_owned(),
        egui::FontData::from_owned(get_font("sarasa-gothic-sc-regular.ttf")),
    );
    fonts.font_data.insert(
        "sarasa_mono".to_owned(),
        egui::FontData::from_owned(get_font("sarasa-mono-sc-regular.ttf")),
    );
    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(Proportional)
        .or_default()
        .insert(0, "sarasa_gothic".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(Monospace)
        .or_default()
        .push("sarasa_mono".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

pub(crate) fn gen_range_rand_num(max_len: usize) -> usize {
    let mut rng = rand::thread_rng();
    let rand_num = rng.gen_range(0..max_len);
    rand_num
}

pub fn random_set_wallpaper(dir_list: Vec<String>) {
    if !dir_list.is_empty() {
        let len = dir_list.len();
        let rand_num = gen_range_rand_num(len);
        let dir_path = dir_list[rand_num].clone();
        match fs::read_dir(dir_path) {
            Ok(dir) => {
                let file_list: Vec<PathBuf> = dir
                    .map(|res| res.map(|e| e.path()))
                    .collect::<Result<Vec<PathBuf>, io::Error>>()
                    .expect("");
                if !file_list.is_empty() {
                    let max_len = file_list.len();
                    let rand_num = gen_range_rand_num(max_len);
                    let wallpaper_path = file_list[rand_num].clone();
                    wallpaper::set_from_path(wallpaper_path.to_str().unwrap())
                        .expect("set wallpaper error");
                }
            }
            Err(err) => {}
        }
    }
}
