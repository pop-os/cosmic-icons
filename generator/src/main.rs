use std::{env, path::PathBuf, process};

mod standard;

fn main() {
    let theme_dirs: Vec<PathBuf> = env::args().skip(1).map(PathBuf::from).collect();
    for (kind, icons) in standard::standard().iter() {
        let mut kind_dirs = Vec::new();
        for size in &["scalable"] {
            for theme_dir in theme_dirs.iter() {
                let kind_dir = theme_dir.join(size).join(kind);
                if kind_dir.is_dir() {
                    kind_dirs.push((size, kind_dir));
                }
            }
        }
        if kind_dirs.is_empty() {
            eprintln!("\x1B[1mfailed to find {kind} folder\x1B[0m");
            continue;
        }
        for icon in icons.iter() {
            let mut found = Vec::new();
            for (size, kind_dir) in kind_dirs.iter() {
                for suffix in &["", "-symbolic"] {
                    for ext in &["png", "svg"] {
                        let icon_path = kind_dir.join(format!("{icon}{suffix}.{ext}"));
                        if icon_path.is_file() {
                            found.push((size, icon_path));
                        }
                    }
                }
            }
            if found.is_empty() {
                eprintln!("\x1B[1mfailed to find {kind}/{icon}\x1B[0m");
            } else {
                println!("found {kind}/{icon}: {:?}", found);
            }
        }
    }
}
