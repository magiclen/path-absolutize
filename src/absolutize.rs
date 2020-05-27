use std::io;
use std::path::{Path, PathBuf};

/// Let `Path` and `PathBuf` have `absolutize` and `absolutize_virtually` method.
pub trait Absolutize {
    /// Get an absolute path. This works even if the path does not exist.
    fn absolutize(&self) -> io::Result<PathBuf>;

    /// Get an absolute path. This works even if the path does not exist.
    fn absolutize_virtually<P: AsRef<Path>>(&self, virtual_root: P) -> io::Result<PathBuf>;
}
