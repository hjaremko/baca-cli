use std::fs::{read, File};
use std::io::{Error, ErrorKind, Write};
use std::path::Path;

pub fn zip_file(path: &Path) -> Result<&Path, Error> {
    let filename = path.file_name().unwrap().to_str().ok_or(ErrorKind::Other)?;
    let path = path.to_str().ok_or(ErrorKind::Other)?;

    println!("Zipping {}.", filename);
    tracing::debug!("Relative path: {}.", path);

    let source = read(path)?;
    let buf = File::create("source.zip")?;
    let mut zip = zip::ZipWriter::new(buf);

    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::DEFLATE);
    zip.start_file(filename, options)?;
    zip.write_all(source.as_ref())?;
    zip.finish()?;

    Ok(Path::new("source.zip"))
}
