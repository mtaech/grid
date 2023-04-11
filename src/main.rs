#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod utils;

use std::{fs, io};
use std::fs::File;
use std::path::PathBuf;
use eframe::egui;
use egui::{FontFamily, FontId, RichText, TextStyle};
use rand::Rng;


fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native("Grid", options, Box::new(|cc| Box::new(Grid::new(cc))))
}

#[derive(Debug)]
struct Grid {
    mode: Mode,
    dir_path: Vec<String>,
}

#[derive(Debug)]
enum Mode {
    RANDOM,
    SHUFFLE,
}

impl Grid {
    fn new(ctx: &eframe::CreationContext) -> Self {
        utils::setup_custom_fonts(&ctx.egui_ctx);
        utils::configure_text_styles(&ctx.egui_ctx);
        Self::default()
    }
    fn count_wallpaper_file(&mut self) {
        if !self.dir_path.is_empty() {
            for path in &self.dir_path {
                match fs::read_dir(path) {
                    Ok(dir) => {
                        for file in dir {

                        }
                    }
                    Err(err) => {}
                }
            }
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            mode: Mode::SHUFFLE,
            dir_path: vec![],
        }
    }
}

impl eframe::App for Grid {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Grid");
            ui.horizontal(|ui| {
                let name_label = ui.label("模式: ");
                let random_mod_btn = ui.button("随机").labelled_by(name_label.id);
                if random_mod_btn.clicked() {
                    self.mode = Mode::RANDOM;
                }
                let shuffle_mode_btn = ui.button("顺序").labelled_by(name_label.id);
                if shuffle_mode_btn.clicked() {
                    self.mode = Mode::SHUFFLE;
                }
            });
            ui.horizontal(|ui| {
                let name_label = ui.label("操作: ");
                let last_wallpaper = ui.button("上一张").labelled_by(name_label.id);
                if last_wallpaper.clicked() {
                    self.mode = Mode::RANDOM;
                }
                let next_wallpaper = ui.button("下一张").labelled_by(name_label.id);
                if next_wallpaper.clicked() {
                    utils::random_set_wallpaper(self.dir_path.clone())
                }
            });
            ui.horizontal(|ui| {
                ui.label("新增壁纸文件夹:");

                if ui.button("选择文件夹:").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        println!("path {:?}", path);
                        let path = path.display().to_string();
                        self.dir_path.push(path);
                        // self.picked_path = Some(path.display().to_string());
                    }
                }
            });
            if !self.dir_path.is_empty() {
                for i in 0..self.dir_path.len() {
                    let dir_path = &self.dir_path.clone()[i];
                    ui.horizontal(|ui| {
                        ui.label(dir_path);

                        if ui.button("delete").clicked() {
                            self.dir_path.remove(i);
                        }
                    });
                }
            }
            ui.label(format!("app mode : {:?}", self.mode));
        });
    }
}
