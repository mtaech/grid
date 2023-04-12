create table image_dir
(
    id          text primary key not null,
    dir_path    text,
    create_time text
);

create table image_file
(
    id          text primary key not null,
    dir_id      text,
    dir_path    text,
    file_path   text,
    create_time text
);