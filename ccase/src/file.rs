//! For the `file` subcommand

use convert_case::Converter;

use std::{fs, io};
use std::path::Path;

enum FileCommandError {
    InvalidPath,
    IOError,
}

pub fn transform_file(path: &Path, conv: Converter, include_ext: bool) -> Result<(), FileCommandError> {
    let cur_filename = path.file_name().ok_or(FileCommandError::InvalidPath)?;

    let new_filename = if include_ext {
        conv.convert(cur_filename.clone().to_string())
    } else {
        let file_stem = path.file_stem().ok_or(FileCommandError::InvalidPath)?;
        let ext = path.extension().ok_or(FileCommandError::InvalidPath)?;
        format!("{}.{}", conv.convert(file_stem), ext.to_string());
    };

    fs::rename(path.as_os_str(), path.as_os_str()).map_err(|_| FileCommandError::IOError)?;

    Ok(())
}
