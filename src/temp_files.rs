use std::fs::File;
use std::io;
use std::process::Stdio;
#[cfg(target_os = "linux")]
use memfile::MemFile;

pub(crate) fn make_cloned_stdio(file: &File) -> Stdio {
    Stdio::from(file.try_clone().unwrap())
}

/// Creates a memfile using the `memfile` crate on Linux
/// or a tempfile using the `tempfile` crate on other systems.
///
/// These files should be deleted automatically when all file descriptors are closed
///
/// Always returns a `File` struct
pub(crate) fn create_temp_file() -> io::Result<File> {
    if cfg!(target_os = "linux") {
        // The file is deleted when all file descriptors are closed
        // https://man7.org/linux/man-pages/man2/memfd_create.2.html
        MemFile::create_default("toster temporary file")
            .map(|memfile| memfile.into_file())
    } else {
        // tempfile() adds FILE_FLAG_DELETE_ON_CLOSE flag on Windows and TMPFILE on Linux
        // so the file should be deleted when all file descriptors are closed
        tempfile::tempfile()
    }
}
