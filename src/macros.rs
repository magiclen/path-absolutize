#[cfg(not(any(feature = "lazy_static_cache", feature = "unsafe_cache")))]
macro_rules! get_cwd_pathbuf {
    () => {
        std::env::current_dir().unwrap()
    };
}

#[cfg(any(feature = "lazy_static_cache", feature = "unsafe_cache"))]
macro_rules! get_cwd_pathbuf {
    () => {
        ()
    };
}

#[cfg(not(any(feature = "lazy_static_cache", feature = "unsafe_cache")))]
macro_rules! get_cwd {
    ($_cwd:expr) => {
        $_cwd.as_path()
    };
}

#[cfg(feature = "lazy_static_cache")]
macro_rules! get_cwd {
    ($_cwd:expr) => {
        $crate::CWD.as_path()
    };
}

#[cfg(feature = "unsafe_cache")]
macro_rules! get_cwd {
    ($_cwd:expr) => {
        unsafe { $crate::CWD.as_path() }
    };
}
