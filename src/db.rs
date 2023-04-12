use crate::entity::ImageDir;
use crate::schema::image_dir::dsl::image_dir;
use crate::schema::image_dir::id;
use crate::DB_CELL;
use chrono::Local;
use diesel::{
    delete, insert_into, select, Connection, Insertable, QueryDsl, RunQueryDsl, SqliteConnection,
    TextExpressionMethods,
};
use home::home_dir;
use nanoid::nanoid;
use serde::de::Unexpected::Str;
use std::fmt::format;
use std::path::PathBuf;
use std::{env, fs};

fn init_app_dir() -> PathBuf {
    let app_dir = home_dir().unwrap().join(".grid");
    if app_dir.exists() {
        fs::create_dir_all(&app_dir).expect("create");
    }
    app_dir
}

pub(crate) fn conn_db() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database url :{}", database_url);
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn add_wallpaper_dir(dir_path: String) {
    let mut conn = conn_db();
    let time_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let dirs = vec![ImageDir {
        id: nanoid!(10),
        dir_path: Option::from(dir_path),
        create_time: Option::from(time_str),
    }];
    insert_into(image_dir)
        .values(&dirs)
        .execute(&mut conn)
        .expect("insert error");
}

pub fn add_wallpaper_file(dir_path: String) {
    let mut conn = conn_db();
    let time_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let dirs = vec![ImageDir {
        id: nanoid!(10),
        dir_path: Option::from(dir_path),
        create_time: Option::from(time_str),
    }];
    insert_into(image_dir)
        .values(&dirs)
        .execute(&mut conn)
        .expect("insert error");
}

pub fn find_all_image_dir() -> Vec<ImageDir> {
    let mut conn = conn_db();
    let vec: Vec<ImageDir> = image_dir
        .load::<ImageDir>(&mut conn)
        .expect("load image dir error");
    vec
}

pub fn delete_image_dir_by_id(dir_id: &str) {
    let mut conn = conn_db();
    delete(image_dir.filter(id.like(dir_id)))
        .execute(&mut conn)
        .expect("delete error");
}
