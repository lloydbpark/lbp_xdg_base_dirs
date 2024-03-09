use color_eyre::{
    // eyre::{bail, ensure, WrapErr},
    eyre::{ ensure, WrapErr},
    Result,
};
use std::{env, fs::DirBuilder, path::PathBuf};

// #[derive(Copy)]
pub enum XdgFileKind {
    Cache,
    Data,
    Config,
    Runtime,
}

pub fn get_xdg_file_path(kind: &XdgFileKind, org: &str, app: &str, file: &str) -> Result<PathBuf> {
    #[cfg(target_family = "unix")]
    let key = "HOME";
    #[cfg(target_family = "windows")]
    let key = "APPDATA";

    let base_dir = env::var(key).wrap_err(format!(
        "The '{key}' environment variable not found while finding XDG paths"
    ))?;
    let mut path = PathBuf::from(base_dir);

    #[cfg(target_family = "unix")]
    match kind {
        XdgFileKind::Cache => path.push(".cache"),
        XdgFileKind::Data => path.push(".local"),
        XdgFileKind::Runtime => {
            path.push(".local");
            path.push("runtime");
        }
        XdgFileKind::Config => path.push(".config"),
    };

    #[cfg(target_family = "windows")]
    match kind {
        XdgFileKind::Cache => path.push("cache"),
        XdgFileKind::Data => path.push("local"),
        XdgFileKind::Runtime => {
            path.push("local");
            path.push("runtime");
        }
        XdgFileKind::Config => path.push("config"),
    };

    path.push(org);
    path.push(app);

    // Create the subdirectories if they don't yet exist
    let mut dir = DirBuilder::new();
    dir.recursive(true);
    dir
        .create(&path)
        .wrap_err("Unable to create the xdg folder!")?;

    ensure!(!file.is_empty(), "An empty file paramater was passed!");

    path.push(file);
    Ok(path)
} // get_xdg_file_path()
