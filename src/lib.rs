extern crate path_dedot;

use std::path::{Path, PathBuf};
use std::io::{self, ErrorKind};

use path_dedot::ParseDot;

/// Current working directory.
pub use path_dedot::CWD;

/// The main separator for the target OS.
pub use path_dedot::MAIN_SEPARATOR;

/// Make `Path` and `PathBuf` have `absolutize` and `absolutize_virtually` method.
pub trait Absolutize {
    /// Get an absolute path. This works even if the path does not exist.
    ///
    /// Please read the following examples to know the parsing rules.
    ///
    /// # Examples
    ///
    /// The dots in a path will be parsed even if it is already an absolute path (which means the path starts with a `MAIN_SEPARATOR` on Unix-like systems).
    ///
    /// ```
    /// extern crate path_absolutize;
    ///
    /// use std::path::Path;
    ///
    /// use path_absolutize::*;
    /// if cfg!(not(windows)) {
    ///
    ///     let p = Path::new("/path/to/123/456");
    ///
    ///     assert_eq!("/path/to/123/456", p.absolutize().unwrap().to_str().unwrap());
    ///
    /// }
    /// ```
    ///
    /// ```
    /// extern crate path_absolutize;
    ///
    /// use std::path::Path;
    ///
    /// use path_absolutize::*;
    ///
    /// if cfg!(not(windows)) {
    ///
    ///     let p = Path::new("/path/to/./123/../456");
    ///
    ///     assert_eq!("/path/to/456", p.absolutize().unwrap().to_str().unwrap());
    ///
    /// }
    /// ```
    ///
    /// If a path starts with a single dot, the dot means **current working directory**.
    ///
    /// ```
    /// extern crate path_absolutize;
    ///
    /// use std::path::Path;
    ///
    /// use path_absolutize::*;
    ///
    /// if cfg!(not(windows)) {
    ///
    ///     let p = Path::new("./path/to/123/456");
    ///
    ///     assert_eq!(Path::join(&CWD, Path::new("path/to/123/456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
    ///
    /// }
    /// ```
    ///
    /// If a path starts with a pair of dots, the dots means the parent of **current working directory**. If **current working directory** is **root**, the parent is still **root**.
    ///
    /// ```
    /// extern crate path_absolutize;
    ///
    /// use std::path::Path;
    ///
    /// use path_absolutize::*;
    ///
    /// if cfg!(not(windows)) {
    ///
    ///     let p = Path::new("../path/to/123/456");
    ///
    ///     let cwd_parent = CWD.parent();
    ///
    ///     match cwd_parent {
    ///        Some(cwd_parent) => {
    ///        assert_eq!(Path::join(&cwd_parent, Path::new("path/to/123/456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
    ///        }
    ///        None => {
    ///            assert_eq!(Path::join(Path::new("/"), Path::new("path/to/123/456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
    ///        }
    ///     }
    ///
    /// }
    /// ```
    ///
    /// A path which does not start with a `MAIN_SEPARATOR`, **Single Dot** and **Double Dots**, will act like having a single dot at the start when `absolutize` method is used.
    ///
    /// ```
    /// extern crate path_absolutize;
    ///
    /// use std::path::Path;
    ///
    /// use path_absolutize::*;
    ///
    /// let p = Path::new("path/to/123/456");
    ///
    /// assert_eq!(Path::join(&CWD, Path::new("path/to/123/456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
    /// ```
    ///
    /// ```
    /// extern crate path_absolutize;
    ///
    /// use std::path::Path;
    ///
    /// use path_absolutize::*;
    ///
    /// if cfg!(not(windows)) {
    ///
    ///     let p = Path::new("path/../../to/123/456");
    ///
    ///     let cwd_parent = CWD.parent();
    ///
    ///     match cwd_parent {
    ///       Some(cwd_parent) => {
    ///            assert_eq!(Path::join(&cwd_parent, Path::new("to/123/456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
    ///       }
    ///       None => {
    ///           assert_eq!(Path::join(Path::new("/"), Path::new("to/123/456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
    ///       }
    /// }
    ///
    /// }
    /// ```
    fn absolutize(&self) -> io::Result<PathBuf>;

    /// Get an absolute path **only under a specific directory**. This works even if the path does not exist.
    ///
    /// Please read the following examples to know the parsing rules.
    ///
    /// # Examples
    ///
    /// The dots in a path will be parsed even if it is already an absolute path (which means the path starts with a `MAIN_SEPARATOR` on Unix-like systems).
    ///
    /// ```
    /// extern crate path_absolutize;
    ///
    /// use std::path::Path;
    ///
    /// use path_absolutize::*;
    ///
    /// if cfg!(not(windows)) {
    ///
    ///     let p = Path::new("/path/to/123/456");
    ///
    ///     assert_eq!("/path/to/123/456", p.absolutize_virtually("/").unwrap().to_str().unwrap());
    ///
    /// }
    /// ```
    ///
    /// ```
    /// extern crate path_absolutize;
    ///
    /// use std::path::Path;
    ///
    /// use path_absolutize::*;
    ///
    /// if cfg!(not(windows)) {
    ///
    ///     let p = Path::new("/path/to/./123/../456");
    ///
    ///     assert_eq!("/path/to/456", p.absolutize_virtually("/").unwrap().to_str().unwrap());
    ///
    /// }
    /// ```
    ///
    /// Every absolute path should under the virtual root.
    ///
    /// ```
    /// extern crate path_absolutize;
    ///
    /// use std::path::Path;
    ///
    /// use std::io::ErrorKind;
    ///
    /// use path_absolutize::*;
    ///
    /// if cfg!(not(windows)) {
    ///
    ///     let p = Path::new("/path/to/123/456");
    ///
    ///     assert_eq!(ErrorKind::InvalidInput, p.absolutize_virtually("/virtual/root").unwrap_err().kind());
    ///
    /// }
    /// ```
    ///
    /// Every relative path should under the virtual root.
    ///
    /// ```
    /// extern crate path_absolutize;
    ///
    /// use std::path::Path;
    ///
    /// use std::io::ErrorKind;
    ///
    /// use path_absolutize::*;
    ///
    /// if cfg!(not(windows)) {
    ///
    ///     let p = Path::new("./path/to/123/456");
    ///
    ///     assert_eq!(ErrorKind::InvalidInput, p.absolutize_virtually("/virtual/root").unwrap_err().kind());
    ///
    /// }
    /// ```
    ///
    /// ```
    /// extern crate path_absolutize;
    ///
    /// use std::path::Path;
    ///
    /// use std::io::ErrorKind;
    ///
    /// use path_absolutize::*;
    ///
    /// if cfg!(not(windows)) {
    ///
    ///     let p = Path::new("../path/to/123/456");
    ///
    ///     assert_eq!(ErrorKind::InvalidInput, p.absolutize_virtually("/virtual/root").unwrap_err().kind());
    ///
    /// }
    /// ```
    ///
    /// A path which does not start with a `MAIN_SEPARATOR`, **Single Dot** and **Double Dots**, will be located in the virtual root after the `absolutize_virtually` method is used.
    ///
    /// ```
    /// extern crate path_absolutize;
    ///
    /// use std::path::Path;
    ///
    /// use path_absolutize::*;
    ///
    /// if cfg!(not(windows)) {
    ///
    ///     let p = Path::new("path/to/123/456");
    ///
    ///     assert_eq!("/virtual/root/path/to/123/456", p.absolutize_virtually("/virtual/root").unwrap().to_str().unwrap());
    ///
    /// }
    /// ```
    ///
    /// ```
    /// extern crate path_absolutize;
    ///
    /// use std::path::Path;
    ///
    /// use path_absolutize::*;
    ///
    /// if cfg!(not(windows)) {
    ///
    ///     let p = Path::new("path/to/../../../../123/456");
    ///
    ///     assert_eq!("/virtual/root/123/456", p.absolutize_virtually("/virtual/root").unwrap().to_str().unwrap());
    ///
    /// }
    /// ```
    fn absolutize_virtually<P: AsRef<Path>>(&self, virtual_root: P) -> io::Result<PathBuf>;
}

impl Absolutize for Path {
    fn absolutize(&self) -> io::Result<PathBuf> {
        if self.is_absolute() {
            self.parse_dot()
        } else {
            let path = Path::join(&CWD, self);

            path.parse_dot()
        }
    }

    #[cfg(not(windows))]
    fn absolutize_virtually<P: AsRef<Path>>(&self, virtual_root: P) -> io::Result<PathBuf> {
        let mut virtual_root = virtual_root.as_ref().absolutize()?;

        if self.is_absolute() {
            let path = self.parse_dot()?;

            if !path.starts_with(&virtual_root) {
                return Err(io::Error::from(ErrorKind::InvalidInput));
            }

            Ok(path)
        } else {
            let path = self.parse_dot()?;

            if path.is_absolute() {
                if !path.starts_with(&virtual_root) {
                    return Err(io::Error::from(ErrorKind::InvalidInput));
                }

                Ok(path)
            } else {
                virtual_root.push(path);

                return Ok(virtual_root);
            }
        }
    }

    #[cfg(windows)]
    fn absolutize_virtually<P: AsRef<Path>>(&self, virtual_root: P) -> io::Result<PathBuf> {
        let mut virtual_root = virtual_root.as_ref().absolutize()?;

        if self.is_absolute() {
            let path = self.parse_dot()?;

            let path_lowercase = path.to_str().unwrap().to_lowercase();

            let virtual_root_lowercase = virtual_root.to_str().unwrap().to_lowercase();

            if !&path_lowercase.starts_with(&virtual_root_lowercase) {
                return Err(io::Error::from(ErrorKind::InvalidInput));
            }

            Ok(path)
        } else {
            let path = self.parse_dot()?;

            if path.is_absolute() {
                let path_lowercase = path.to_str().unwrap().to_lowercase();

                let virtual_root_lowercase = virtual_root.to_str().unwrap().to_lowercase();

                if !&path_lowercase.starts_with(&virtual_root_lowercase) {
                    return Err(io::Error::from(ErrorKind::InvalidInput));
                }

                Ok(path)
            } else {
                virtual_root.push(path);

                return Ok(virtual_root);
            }
        }
    }
}

impl Absolutize for PathBuf {
    fn absolutize(&self) -> io::Result<PathBuf> {
        let path = Path::new(&self);

        path.absolutize()
    }

    fn absolutize_virtually<P: AsRef<Path>>(&self, virtual_root: P) -> io::Result<PathBuf> {
        let path = Path::new(&self);

        path.absolutize_virtually(virtual_root)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::io::ErrorKind;
    use super::*;

    #[test]
    #[cfg(not(windows))]
    fn absolutize_lv0_1() {
        let p = Path::new("/path/to/123/456");

        assert_eq!("/path/to/123/456", p.absolutize().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(windows)]
    fn absolutize_lv0_1() {
        let p = Path::new(r"\path\to\123\456");

        assert_eq!(Path::join(&CWD, Path::new(r"path\to\123\456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn absolutize_lv0_2() {
        let p = Path::new("/path/to/./123/../456");

        assert_eq!("/path/to/456", p.absolutize().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(windows)]
    fn absolutize_lv0_2() {
        let p = Path::new(r"\path\to\.\123\..\456");

        assert_eq!(Path::join(&CWD, Path::new(r"path\to\456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn absolutize_lv1_1() {
        let p = Path::new("./path/to/123/456");

        assert_eq!(Path::join(&CWD, Path::new("path/to/123/456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(windows)]
    fn absolutize_lv1_1() {
        let p = Path::new(r".\path\to\123\456");

        assert_eq!(Path::join(&CWD, Path::new(r"path\to\123\456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn absolutize_lv1_2() {
        let p = Path::new("../path/to/123/456");

        let cwd_parent = CWD.parent();

        match cwd_parent {
            Some(cwd_parent) => {
                assert_eq!(Path::join(&cwd_parent, Path::new("path/to/123/456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
            }
            None => {
                assert_eq!(Path::join(Path::new("/"), Path::new("path/to/123/456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
            }
        }
    }

    #[test]
    #[cfg(windows)]
    fn absolutize_lv1_2() {
        let p = Path::new(r"..\path\to\123\456");

        let cwd_parent = CWD.parent();

        match cwd_parent {
            Some(cwd_parent) => {
                assert_eq!(Path::join(&cwd_parent, Path::new(r"path\to\123\456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
            }
            None => {
                assert_eq!(Path::join(Path::new(CWD.get_path_prefix().unwrap().as_os_str()), Path::new(r"path\to\123\456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
            }
        }
    }

    #[test]
    #[cfg(not(windows))]
    fn absolutize_lv2() {
        let p = Path::new("path/to/123/456");

        assert_eq!(Path::join(&CWD, Path::new("path/to/123/456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(windows)]
    fn absolutize_lv2() {
        let p = Path::new(r"path\to\123\456");

        assert_eq!(Path::join(&CWD, Path::new(r"path\to\123\456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn absolutize_lv3() {
        let p = Path::new("path/../../to/123/456");

        let cwd_parent = CWD.parent();

        match cwd_parent {
            Some(cwd_parent) => {
                assert_eq!(Path::join(&cwd_parent, Path::new("to/123/456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
            }
            None => {
                assert_eq!(Path::join(Path::new("/"), Path::new("to/123/456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
            }
        }
    }

    #[test]
    #[cfg(windows)]
    fn absolutize_lv3() {
        let p = Path::new(r"path\..\..\to\123\456");

        let cwd_parent = CWD.parent();

        match cwd_parent {
            Some(cwd_parent) => {
                assert_eq!(Path::join(&cwd_parent, Path::new(r"to\123\456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
            }
            None => {
                assert_eq!(Path::join(Path::new(CWD.get_path_prefix().unwrap().as_os_str()), Path::new(r"to\123\456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
            }
        }
    }

    #[test]
    #[cfg(windows)]
    fn absolutize_lv4() {
        let cwd_prefix = CWD.get_path_prefix().unwrap();

        let target_prefix = if cwd_prefix.as_os_str().ne("C:") {
            "C:"
        } else {
            "D:"
        };

        let target = Path::new(format!(r"{}123\567", target_prefix));

        let cwd = CWD.to_str().unwrap();

        let path = PathBuf::from(format!("{}{}", target_prefix, &cwd[cwd_prefix.as_os_str().len()..]));

        assert_eq!(path.to_str().unwrap(), target.absolutize().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn virtually_absolutize_lv0_1() {
        let p = Path::new("/path/to/123/456");

        assert_eq!("/path/to/123/456", p.absolutize_virtually("/").unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn virtually_absolutize_lv0_2() {
        let p = Path::new("/path/to/./123/../456");

        assert_eq!("/path/to/456", p.absolutize_virtually("/").unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn virtually_absolutize_lv0_3() {
        let p = Path::new("/path/to/123/456");

        assert_eq!(ErrorKind::InvalidInput, p.absolutize_virtually("/virtual/root").unwrap_err().kind());
    }

    #[test]
    #[cfg(not(windows))]
    fn virtually_absolutize_lv1_1() {
        let p = Path::new("./path/to/123/456");

        assert_eq!(ErrorKind::InvalidInput, p.absolutize_virtually("/virtual/root").unwrap_err().kind());
    }

    #[test]
    #[cfg(not(windows))]
    fn virtually_absolutize_lv1_2() {
        let p = Path::new("../path/to/123/456");

        assert_eq!(ErrorKind::InvalidInput, p.absolutize_virtually("/virtual/root").unwrap_err().kind());
    }

    #[test]
    #[cfg(not(windows))]
    fn virtually_absolutize_lv2() {
        let p = Path::new("path/to/123/456");

        assert_eq!("/virtual/root/path/to/123/456", p.absolutize_virtually("/virtual/root").unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn virtually_absolutize_lv3() {
        let p = Path::new("path/to/../../../../123/456");

        assert_eq!("/virtual/root/123/456", p.absolutize_virtually("/virtual/root").unwrap().to_str().unwrap());
    }
}
