//! # zip module
//!
//! `zip` is utility module hosting the zip & unzip functions

use crate::errors::Result;
use log::debug;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use zip::result::ZipError;
use zip::write::FileOptions;

/// Zips source folder into destination file
///
/// Arguments:
///
/// * `it`: iterator of files within directory (retrieved from walkdir::WalkDir)
/// * `prefix`: source directory
/// * `writer`: file wirter/handle
/// 
/// Returns
///  zip::result::ZipResult<()>
fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            debug!("adding file {:?} as {:?} ...", path, name);
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            debug!("adding dir {:?} as {:?} ...", path, name);
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}

/// Zips source folder into destination file
///
/// Arguments:
///
/// * `src_dir`: source directory to zip
/// * `dst_file`: path to destination zip file
/// 
/// Returns
///  zip::result::ZipResult<()>
pub fn zip_directory(src_dir: &str, dst_file: &str) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src_dir.to_string());
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file)?;

    Ok(())
}

/// Unzips source zip file into destination folder
///
/// Arguments:
///
/// * `zip_path`: path to zip file to unzip
/// * `target_folder`: destination folder
/// 
/// Returns
///  Result<()>
pub fn unzip_file(zip_path: &str, target_folder: &str) -> Result<()> {
    let fname = std::path::Path::new(zip_path);
    let file = fs::File::open(&fname)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let base_path = Path::new(target_folder);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = base_path.join(file.sanitized_name());

        {
            let comment = file.comment();
            if !comment.is_empty() {
                debug!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            debug!(
                "File {} extracted to \"{}\"",
                i,
                outpath.as_path().display()
            );
            fs::create_dir_all(&outpath)?;
        } else {
            debug!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.as_path().display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}
