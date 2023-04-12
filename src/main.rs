#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod db;
mod entity;
mod schema;
mod utils;

use diesel::SqliteConnection;
use eframe::egui;
use eframe::glow::Context;
use egui::{FontFamily, FontId, RichText, TextStyle};
use once_cell::sync::OnceCell;
use rand::Rng;
use std::fs::File;
use std::path::PathBuf;
use std::{env, fs, io};
use crate::db::delete_image_dir_by_id;

const DB_CELL: OnceCell<SqliteConnection> = OnceCell::new();

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    dotenvy::dotenv().ok().expect("ok");
    println!("{}", env::var("DATABASE_URL").expect("hh"));
    let conn = db::conn_db();
    DB_CELL.set(conn);
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
    event_receiver: Option<SystrayEventReceiver>,
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
                    Ok(dir) => for file in dir {},
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
            event_receiver:None
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
                        self.dir_path.push(path.clone());
                        db::add_wallpaper_dir(path)
                    }
                }
            });
            let image_dirs = db::find_all_image_dir();
            if !image_dirs.is_empty() {
                for dir in image_dirs {
                    ui.horizontal(|ui| {
                        ui.label(dir.dir_path.unwrap());

                        if ui.button("delete").clicked() {
                            // self.dir_path.remove(i);
                            delete_image_dir_by_id(&dir.id);
                        }
                    });
                }
            }
            ui.label(format!("app mode : {:?}", self.mode));
        });
    }
    fn on_exit(&mut self, _gl: Option<&Context>) {}
}
