use crate::schema::{image_dir, image_file};
use diesel::{Insertable, QueryId, Queryable, Selectable};
use serde::Deserialize;

#[derive(Deserialize, Insertable, Queryable, Selectable, QueryId, Clone)]
#[diesel(table_name = image_dir)]
pub struct ImageDir {
    pub(crate) id: String,
    pub(crate) dir_path: Option<String>,
    pub(crate) create_time: Option<String>,
}

#[derive(Deserialize, Insertable, Queryable, Selectable, QueryId, Clone)]
#[diesel(table_name = image_file)]
pub struct ImageFile {
    id: String,
    dir_path: Option<String>,
    create_time: Option<String>,
}
