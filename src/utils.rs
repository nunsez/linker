use std::path::{Path, PathBuf};

pub fn walkdir<F>(source_path: &Path, destination_path: &Path, if_ok: F)
where
    F: Fn(&Path, &Path),
{
    let Ok(entries) = source_path.read_dir() else {
        return;
    };

    for entry in entries {
        match entry {
            Ok(entry) => {
                let src_path = entry.path();

                match src_path.strip_prefix(source_path) {
                    Ok(file_name) => {
                        let dst_path = destination_path.join(file_name);
                        if_ok(&src_path, &dst_path)
                    }
                    Err(e) => eprintln!("{e}"),
                }
            }
            Err(e) => eprintln!("{e}"),
        }
    }
}

pub fn absolute(relative: &Path, base: &Path) -> PathBuf {
    let abs = base.join(relative);
    path_clean::clean(abs)
}
