use log::{debug, error, info, trace, warn};
use std::fs::read_dir;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct FolderToScan {
    pub(crate) path: String,
    pub(crate) recursive: bool,
}

impl FolderToScan {
    pub fn scan(&self) -> Result<u64, std::io::Error> {
        scan_folder(Path::new(&self.path), self.recursive)
    }
}

#[inline]
fn scan_folder(dir: &Path, is_recursive: bool) -> Result<u64, std::io::Error> {
    let mut tot: u64 = 0;
    for entry in read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() && is_recursive {
            tot += scan_folder(&entry.path(), is_recursive)?;
        } else {
            tot += entry.metadata()?.len();
        }
    }
    Ok(tot)
}

#[derive(Debug, Clone)]
pub(crate) struct FolderWithSize {
    pub folder: FolderToScan,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub(crate) struct FolderScanner {
    folders: Vec<FolderToScan>,
}

impl FolderScanner {
    pub fn folders(&self) -> &[FolderToScan] {
        &self.folders
    }

    pub fn from_json(json: &str) -> Result<FolderScanner, serde_json::error::Error> {
        Ok(FolderScanner {
            folders: serde_json::from_str(json)?,
        })
    }

    pub fn scan(&self) -> Result<Vec<FolderWithSize>, std::io::Error> {
        let mut v_sizes = Vec::new();

        for folder in &self.folders {
            trace!("scanning folder {:?}", folder);
            let size = folder.scan()?;
            debug!("folder {:?}, size == {}", folder, size);
            v_sizes.push(FolderWithSize {
                folder: folder.clone(),
                size,
            });
        }
        Ok(v_sizes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let s = "
		  	 [
		  		 { \"path\": \"pippo\", \"recursive\": true },
		  		 { \"path\": \"pluto\", \"recursive\": true }, 
		  		 { \"path\": \"paperino\", \"recursive\": false } 
		  	]
		  ";

        let dresp: FolderScanner = FolderScanner::from_json(s).unwrap();

        assert_eq!(dresp.folders().len(), 3);
        assert_eq!(dresp.folders()[0].recursive, true);
        assert_eq!(dresp.folders()[2].path, "paperino");
    }

}
