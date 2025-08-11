use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn package_list(target_path: &PathBuf) -> Vec<String> {
    let Ok(entries) = fs::read_dir(target_path) else {
        return Vec::new();
    };

    entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.is_dir())
        .filter_map(|dir| Some(dir.file_name()?.to_str()?.to_owned()))
        .collect()
}

pub fn traverse<F>(source_path: &Path, destination_path: &Path, if_ok: F)
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
                    Err(e) => println!("{e}"),
                }
            }
            Err(e) => println!("{e}"),
        }
    }
}

#[cfg(test)]
pub(crate) fn touch(path: &Path) {
    let parent = path.parent().unwrap();
    ensure_exist(parent);
    fs::write(path, "").unwrap();
}

#[cfg(test)]
pub(crate) fn ensure_exist<P>(path: P)
where
    P: AsRef<Path>,
{
    fs::create_dir_all(path).unwrap();
}

#[cfg(test)]
pub(crate) const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures");
