use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Read};
use std::ffi;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil,
    FailedToGetExePath,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub struct FileReader {
    root_path: PathBuf,
}

impl FileReader {
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<FileReader, Error> {
        let exe_file_name = ::std::env::current_exe()
        .map_err(|_| Error::FailedToGetExePath)?;
        let exe_path = exe_file_name.parent()
            .ok_or(Error::FailedToGetExePath)?;
        Ok(FileReader {
            root_path: exe_path.join(rel_path)
        })
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(
            resource_name_to_path(&self.root_path, resource_name)
        )?;

        // allocate buffer of the same size as file
        let mut buffer: Vec<u8> = Vec::with_capacity(
            file.metadata()?.len() as usize + 1
        );
        file.read_to_end(&mut buffer)?;

        // check for nul byte
        // dereference twice with **
        // once for the iterator, and once for the |i| enclosure
        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(Error::FileContainsNil);
        }

        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}
