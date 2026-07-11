use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    io::{self, ErrorKind},
    path::{Component, Path, PathBuf, MAIN_SEPARATOR_STR},
};

use crate::{path_dedot::ParseDot, Absolutize};

impl Absolutize for Path {
    #[inline]
    fn absolutize(&self) -> io::Result<Cow<'_, Path>> {
        if self.is_absolute() {
            // an absolute path never reads the cwd, so an empty one is enough here
            Ok(self.absolutize_from(Path::new("")))
        } else {
            let cwd = get_cwd!();

            Ok(self.absolutize_from(cwd))
        }
    }

    fn absolutize_from(&self, cwd: impl AsRef<Path>) -> Cow<'_, Path> {
        let main_separator = OsStr::new(MAIN_SEPARATOR_STR);

        let mut iter = self.components();

        let mut has_change = false;

        if let Some(first_component) = iter.next() {
            let mut tokens = Vec::new();

            let first_is_root = match first_component {
                Component::RootDir => {
                    tokens.push(main_separator);

                    true
                },
                Component::CurDir => {
                    has_change = true;

                    let cwd = cwd.as_ref();

                    for token in cwd.iter() {
                        tokens.push(token);
                    }

                    !tokens.is_empty() && tokens[0] == main_separator
                },
                Component::ParentDir => {
                    has_change = true;

                    let cwd = cwd.as_ref();

                    match cwd.parent() {
                        Some(cwd_parent) => {
                            for token in cwd_parent.iter() {
                                tokens.push(token);
                            }

                            !tokens.is_empty() && tokens[0] == main_separator
                        },
                        None => {
                            // don't care about `cwd` is "//" or "///"
                            if cwd == main_separator {
                                tokens.push(main_separator);

                                true
                            } else {
                                false
                            }
                        },
                    }
                },
                _ => {
                    has_change = true;

                    let cwd = cwd.as_ref();

                    for token in cwd.iter() {
                        tokens.push(token);
                    }

                    let first_is_root = !tokens.is_empty() && tokens[0] == main_separator;

                    tokens.push(first_component.as_os_str());

                    first_is_root
                },
            };

            for component in iter {
                match component {
                    Component::CurDir => {
                        // may be unreachable
                        has_change = true;
                    },
                    Component::ParentDir => {
                        let tokens_length = tokens.len();

                        if tokens_length > 0 && (tokens_length != 1 || !first_is_root) {
                            tokens.pop();
                        }

                        has_change = true;
                    },
                    _ => {
                        tokens.push(component.as_os_str());
                    },
                }
            }

            // all tokens can be removed by double dots when the given cwd is relative, yielding an empty path
            if tokens.is_empty() {
                return Cow::from(PathBuf::new());
            }

            let tokens_length = tokens.len();

            let mut size = tokens.iter().fold(tokens_length - 1, |acc, &x| acc + x.len());

            if first_is_root && tokens_length > 1 {
                size -= 1;
            }

            if has_change || size != self.as_os_str().len() {
                let mut path_string = OsString::with_capacity(size);

                let mut iter = tokens.iter();

                path_string.push(iter.next().unwrap());

                if tokens_length > 1 {
                    if !first_is_root {
                        path_string.push(main_separator);
                    }

                    for token in iter.take(tokens_length - 2) {
                        path_string.push(token);

                        path_string.push(main_separator);
                    }

                    path_string.push(tokens[tokens_length - 1]);
                }

                let path_buf = PathBuf::from(path_string);

                Cow::from(path_buf)
            } else {
                Cow::from(self)
            }
        } else {
            Cow::from(cwd.as_ref().to_owned())
        }
    }

    fn absolutize_virtually(&self, virtual_root: impl AsRef<Path>) -> io::Result<Cow<'_, Path>> {
        let virtual_root = virtual_root.as_ref().absolutize()?;

        let path = self.parse_dot()?;

        if path.is_absolute() {
            if !path.starts_with(&virtual_root) {
                return Err(io::Error::from(ErrorKind::InvalidInput));
            }

            Ok(path)
        } else {
            let mut virtual_root = virtual_root.into_owned();

            virtual_root.push(path);

            Ok(Cow::from(virtual_root))
        }
    }
}
