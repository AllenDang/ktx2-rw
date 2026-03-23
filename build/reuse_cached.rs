use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::constant::KTX_SOFTWARE_VERSION;

const PACKAGE_PREFIX: &str = "ktx2-rw-";
const OUTPUT_DIR_NAME: &str = "out";
const BUILD_DIR_NAME: &str = "KTX-Software-build";
const BINDINGS_FILE_NAME: &str = "bindings.rs";

pub fn try_reuse_cached_build(out_dir: &Path) -> bool {
    find_cached_build(out_dir)
        .and_then(|cache| cache.validate().then_some(cache))
        .and_then(|cache| cache.restore_to(out_dir).then_some(()))
        .is_some()
}

struct CachedBuild {
    ktx_build_dir: PathBuf,
    ktx_source_dir: PathBuf,
    bindings_file: PathBuf,
}

impl CachedBuild {
    fn from_package_dir(package_dir: &Path) -> Self {
        let output_dir = package_dir.join(OUTPUT_DIR_NAME);
        Self {
            ktx_build_dir: output_dir.join(BUILD_DIR_NAME),
            ktx_source_dir: output_dir.join(format!("KTX-Software-{}", KTX_SOFTWARE_VERSION)),
            bindings_file: output_dir.join(BINDINGS_FILE_NAME),
        }
    }

    fn validate(&self) -> bool {
        self.ktx_build_dir.exists() && self.ktx_source_dir.exists() && self.bindings_file.exists()
    }

    fn restore_to(&self, target: &Path) -> bool {
        copy_directory(&self.ktx_build_dir, &target.join(BUILD_DIR_NAME))
            && copy_directory(
                &self.ktx_source_dir,
                &target.join(format!("KTX-Software-{}", KTX_SOFTWARE_VERSION)),
            )
            && fs::copy(&self.bindings_file, target.join(BINDINGS_FILE_NAME)).is_ok()
    }
}

fn find_cached_build(out_dir: &Path) -> Option<CachedBuild> {
    let build_root = out_dir.parent()?.parent()?;
    let entries = fs::read_dir(build_root).ok()?;

    entries
        .filter_map(Result::ok)
        .filter(|entry| is_candidate_package(entry.path(), out_dir))
        .map(|entry| CachedBuild::from_package_dir(&entry.path()))
        .find(CachedBuild::validate)
}

fn is_candidate_package(path: PathBuf, current_out_dir: &Path) -> bool {
    let file_name = match path.file_name() {
        Some(name) => name.to_string_lossy(),
        None => return false,
    };
    let parent = match current_out_dir.parent() {
        Some(p) => p,
        None => return false,
    };
    file_name.starts_with(PACKAGE_PREFIX) && path != parent
}

fn copy_directory(source: &Path, destination: &Path) -> bool {
    if fs::create_dir_all(destination).is_err() {
        return false;
    }
    let Ok(entries) = fs::read_dir(source) else {
        return false;
    };
    entries.filter_map(Result::ok).all(|entry| {
        let src = entry.path();
        let dst = destination.join(entry.file_name());
        entry.file_type().is_ok_and(|ty| {
            if ty.is_dir() {
                copy_directory(&src, &dst)
            } else {
                fs::copy(&src, &dst).is_ok()
            }
        })
    })
}
