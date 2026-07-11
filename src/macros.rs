#[cfg(not(feature = "fixed_workdir"))]
macro_rules! get_cwd {
    () => {
        std::env::current_dir()?
    };
}

#[cfg(feature = "fixed_workdir")]
macro_rules! get_cwd {
    () => {
        $crate::CWD.as_path()
    };
}
