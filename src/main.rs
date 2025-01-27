use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::format_err;
use anyhow::Result;
use async_zip::tokio::read::fs::ZipFileReader as FsZipFileReader;

#[tokio::main]
pub async fn main() -> Result<()> {
    let start = Instant::now();
    let archive = get_webxdc_archive("/home/jonathan/huge.xdc".as_ref()).await?;

    println!("{:?}", start.elapsed());
    let blob = get_blob(&archive, "icon.png").await?;
    println!("{}", blob[1]);
    println!("{:?}", start.elapsed());

    Ok(())
}

async fn get_blob(archive: &FsZipFileReader, name: &str) -> Result<Vec<u8>> {
    let (i, _) = find_zip_entry(archive.file(), name)
        .ok_or_else(|| anyhow!("no entry found for {}", name))?;
    let mut reader = archive.reader_with_entry(i).await?;
    let mut buf = Vec::new();
    reader.read_to_end_checked(&mut buf).await?;
    Ok(buf)
}

fn find_zip_entry<'a>(
    file: &'a async_zip::ZipFile,
    name: &str,
) -> Option<(usize, &'a async_zip::StoredZipEntry)> {
    for (i, ent) in file.entries().iter().enumerate() {
        if ent.filename().as_bytes() == name.as_bytes() {
            return Some((i, ent));
        }
    }
    None
}

/// Get handle to a webxdc ZIP-archive.
/// To check for file existence use archive.by_name(), to read a file, use get_blob(archive).
async fn get_webxdc_archive(path: &Path) -> Result<FsZipFileReader> {
    let archive = FsZipFileReader::new(path).await?;
    Ok(archive)
}
