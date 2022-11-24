use std::path::{Path, PathBuf};

use shellexpand::tilde;

pub(crate) fn expand_user<S: AsRef<str>>(path: S) -> PathBuf {
    let t = tilde(path.as_ref());
    Path::new(t.as_ref()).to_path_buf()

}