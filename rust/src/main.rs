use chrono::{DateTime, Local}; // For formatting modification time
use std::env;
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt}; // For Unix-specific metadata methods and permissions extension
use std::path::Path;
use users::{get_group_by_gid, get_user_by_uid};

fn main() {
    const FOLDER_ICON: &str = "\u{ea83}";
    const FILE_ICON: &str = "\u{f15b}";
    const RUST_ICON: &str = "\u{e7a8}";
    const TEXT_ICON: &str = "\u{f0f6}";
    const JSON_ICON: &str = "\u{eb0f}";
    const TOML_ICON: &str = "\u{e6b2}";
    const MD_ICON: &str = "\u{e73e}";
    const HTML_ICON: &str = "\u{e736}";
    const CSS_ICON: &str = "\u{e749}";
    const JS_ICON: &str = "\u{f2ee}";
    const PY_ICON: &str = "\u{e73c}";
    const YAML_ICON: &str = "\u{e60e}";
    const GITIGNORE_ICON: &str = "\u{e65d}";

    let args: Vec<String> = env::args().collect();
    let show_hidden = args.contains(&String::from("-a"));
    let show_list = args.contains(&String::from("-l"));

    if let Ok(entries) = fs::read_dir(".") {
        let mut files_and_dirs: Vec<String> = Vec::new();
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                let metadata = entry.metadata().unwrap();
                let is_hidden = file_name.starts_with('.');
                if !show_hidden && is_hidden {
                    continue;
                }
                let icon = if metadata.is_dir() {
                    if file_name == "node_modules" {
                        "\u{e5fa}"
                    } else if file_name == ".git" {
                        "\u{e5fb}"
                    } else {
                        FOLDER_ICON
                    }
                } else {
                    if file_name == ".gitignore" {
                        GITIGNORE_ICON
                    } else {
                        match Path::new(&file_name).extension().and_then(|s| s.to_str()) {
                            Some("rs") => RUST_ICON,
                            Some("txt") => TEXT_ICON,
                            Some("json") => JSON_ICON,
                            Some("toml") => TOML_ICON,
                            Some("md") => MD_ICON,
                            Some("html") => HTML_ICON,
                            Some("css") => CSS_ICON,
                            Some("js") => JS_ICON,
                            Some("py") => PY_ICON,
                            Some("yaml") | Some("yml") => YAML_ICON,
                            _ => FILE_ICON,
                        }
                    }
                };

                if show_list {
                    println!("total {}", files_and_dirs.len());
                    for entry in &files_and_dirs {
                        let path = Path::new(&entry);
                        let metadata = fs::metadata(&path).unwrap();
                        let permissions = metadata.permissions();
                        let file_type = if metadata.is_dir() { "d" } else { "-" };
                        let permissions_string =
                            format!("{}{:o}", file_type, permissions.mode() & 0o777);
                        let user_name = get_user_by_uid(metadata.uid())
                            .map(|u| u.name().to_string_lossy().into_owned())
                            .unwrap_or("unknown".to_string());
                        let group_name = get_group_by_gid(metadata.gid())
                            .map(|g| g.name().to_string_lossy().into_owned())
                            .unwrap_or("unknown".to_string());
                        let modified: DateTime<Local> =
                            DateTime::from(metadata.modified().unwrap());
                        let size = metadata.len();

                        println!(
                            "{} {} {} {} {} {}",
                            permissions_string,
                            user_name,
                            group_name,
                            size,
                            modified.format("%d. %b %H:%M"),
                            path.file_name().unwrap().to_string_lossy()
                        );
                    }
                } else {
                    files_and_dirs.push(format!(
                        "{} {}{}",
                        icon,
                        file_name,
                        if metadata.is_dir() { "/" } else { "" }
                    ));
                }
            }
        }
        println!("{}", files_and_dirs.join("  "));
    }
}
