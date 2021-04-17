use std::fs::{read, File};
use std::io::Write;
use std::path::Path;

pub fn zip_file(path: &Path) -> Option<String> {
    let filename = path.file_name().unwrap().to_str()?;
    let path = path.to_str()?;

    println!("Zipping {}.", filename);
    tracing::debug!("Relative path: {}.", path);

    let source = read(path).ok()?;
    let buf = File::create("source.zip").ok()?;
    let mut zip = zip::ZipWriter::new(buf);

    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    zip.start_file(filename, options).ok()?;
    zip.write(source.as_ref()).ok()?;
    zip.finish().ok()?;

    Some("source.zip".to_string())
}
