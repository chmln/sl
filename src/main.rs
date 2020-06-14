mod error;

use clap::Clap;
use error::{Error, Result};
use std::path::PathBuf;

#[derive(Clap)]
#[clap(version = clap::crate_version!())]
#[clap(about = r#"Symlinks, improved.

EXAMPLES:
    create a symlink at ./my_path pointing to /other/path
    $ sl ./my_path /other/path"#)]
pub struct Cmd {
    /// where to place the symlink (a relative or absolute path)
    link_name: PathBuf,
    /// where the symlink points to (a relative or absolute path)
    pointing_to: PathBuf,
    #[clap(long)]
    /// removes existing `<link_name>`, if any. WARNING: be careful
    force: bool,
}

fn main() -> Result<()> {
    let Cmd {
        pointing_to,
        link_name,
        force,
    } = Cmd::parse();

    let pointing_to = pointing_to.canonicalize()?;

    if !pointing_to.exists() {
        return Err(Error::SourceDoesNotExist(pointing_to.clone()));
    }

    if let Some(parent) = link_name.parent() {
        std::fs::create_dir_all(parent)?;
    }

    match (link_name.exists(), link_name.is_dir(), force) {
        (true, true, true) => std::fs::remove_dir_all(&link_name)?,
        (true, false, true) => std::fs::remove_file(&link_name)?,
        (true, _, false) => return Err(Error::TargetExists),
        _ => {}
    };

    std::os::unix::fs::symlink(pointing_to, link_name)?;

    Ok(())
}
