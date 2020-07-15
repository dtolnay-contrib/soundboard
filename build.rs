use std::path::Path;
use std::{env, fs};

fn main() {
    let target_dir_path = env::var("OUT_DIR").unwrap();
    copy_file(&target_dir_path, "soundboard.toml");
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;
    copy_options.skip_exist = true;
    if Path::new("soundboards").exists() {
        fs_extra::dir::copy(
            "soundboards",
            Path::new(&target_dir_path).join("..").join("..").join(".."),
            &copy_options,
        )
        .expect("copy failed");
    }
    if Path::new("web").exists() {
        fs_extra::dir::copy(
            "web",
            Path::new(&target_dir_path).join("..").join("..").join(".."),
            &copy_options,
        )
        .expect("copy failed");
    }
}

fn copy_file<S: AsRef<std::ffi::OsStr> + ?Sized, P: Copy + AsRef<Path>>(
    target_dir_path: &S,
    file_name: P,
) {
    fs::copy(
        file_name,
        Path::new(&target_dir_path).join("../../..").join(file_name),
    )
    .unwrap();
}
