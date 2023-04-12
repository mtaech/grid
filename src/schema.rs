// @generated automatically by Diesel CLI.

diesel::table! {
    image_dir (id) {
        id -> Text,
        dir_path -> Nullable<Text>,
        create_time -> Nullable<Text>,
    }
}

diesel::table! {
    image_file (id) {
        id -> Text,
        dir_id -> Nullable<Text>,
        dir_path -> Nullable<Text>,
        file_path -> Nullable<Text>,
        create_time -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(image_dir, image_file,);
