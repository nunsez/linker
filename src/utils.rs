use std::{
    fs,
    os::unix,
    path::{Path, PathBuf},
};

pub fn package_list(target_path: &Path) -> Vec<String> {
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

pub fn link_tree(original: &Path, link: &Path, simulate: bool) {
    if !link.exists() || original.is_file() {
        create_symlink(original, link, simulate);
        return;
    }

    walkdir(original, link, |orig, lnk| link_tree(orig, lnk, simulate));
}

pub fn unlink_tree(original: &Path, link: &Path, simulate: bool) {
    if link.is_symlink() {
        remove_symlink(original, link, simulate);
        return;
    }

    walkdir(original, link, |orig, lnk| unlink_tree(orig, lnk, simulate));
}

fn create_symlink(original: &Path, link: &Path, simulate: bool) {
    if link.exists() {
        eprintln!("File exists and will not be symlinked: {}", link.display());
        return;
    }

    let Some(link_parent) = link.parent() else {
        eprintln!("Failed to get parent directory for {}", link.display());
        return;
    };

    let original_relative =
        pathdiff::diff_paths(original, link_parent).unwrap_or_else(|| original.to_path_buf());

    println!(
        "LINK: {} => {}",
        link.display(),
        original_relative.display()
    );

    if simulate {
        return;
    };

    if let Err(e) = unix::fs::symlink(original_relative, link) {
        eprintln!("LINK ERROR: {e}");
    }
}

fn remove_symlink(original: &Path, link: &Path, simulate: bool) {
    if !link.exists() || !link.is_symlink() {
        return;
    }

    let Ok(link_target) = fs::read_link(link) else {
        return;
    };

    let Some(link_parent) = link.parent() else {
        eprintln!("Failed to get parent directory for {}", link.display());
        return;
    };

    let link_target = absolute(&link_target, link_parent);

    if link_target != original {
        return;
    }

    println!("UNLINK: {}", link.display());

    if simulate {
        return;
    }

    if let Err(e) = fs::remove_file(link) {
        eprintln!("UNLINK ERROR: {e}");
    }
}
