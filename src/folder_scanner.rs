use std::fs::read_dir;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub(crate) enum RecurseType {
    None,
    Sum,
    Explode,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct FolderToScan {
    pub(crate) path: String,
    pub(crate) explode_depth: i32,
    pub(crate) sum_remaining_subfolders: bool,
    pub(crate) user: Option<String>,
}

#[inline]
fn scan_folder_explode(
    folder_to_scan: &FolderToScan,
) -> Result<Vec<FolderWithSize>, std::io::Error> {
    log::trace!("scan_folder_explode({:?})", &folder_to_scan);
    let mut tot: u64 = 0;
    let mut v = Vec::new();

    for entry in read_dir(&folder_to_scan.path)? {
        let entry = entry?;

        // let's create the inner folder to scan.
        // If the explode_depth is bigger than zero we
        // decrement it and explode recursively.
        // If it's -1 we explode without stopping.
        // If it's 0 we just sum the contents and stop
        // exploding.
        let folder_inner = FolderToScan {
            path: entry.path().to_str().unwrap().to_owned(),
            explode_depth: if folder_to_scan.explode_depth > 0 {
                folder_to_scan.explode_depth - 1
            } else {
                folder_to_scan.explode_depth
            },
            sum_remaining_subfolders: folder_to_scan.sum_remaining_subfolders,
            user: folder_to_scan.user.to_owned(),
        };

        if entry.file_type()?.is_dir() {
            if folder_to_scan.explode_depth == -1 || folder_to_scan.explode_depth > 1 {
                v.extend_from_slice(&scan_folder_explode(&folder_inner)?);
            } else {
                v.push(scan_folder_sum(&folder_inner)?);
            }
        } else {
            tot += entry.metadata()?.len();
        }
    }

    v.push(FolderWithSize {
        folder: folder_to_scan.to_owned(),
        size: tot,
    });

    Ok(v)
}

#[inline]
fn scan_folder_sum(folder_to_scan: &FolderToScan) -> Result<FolderWithSize, std::io::Error> {
    log::trace!("scan_folder_sum({:?})", &folder_to_scan);
    let mut tot: u64 = 0;

    for entry in read_dir(&folder_to_scan.path)? {
        let entry = entry?;

        let folder_inner = FolderToScan {
            path: entry.path().to_str().unwrap().to_owned(),
            explode_depth: folder_to_scan.explode_depth,
            sum_remaining_subfolders: folder_to_scan.sum_remaining_subfolders,
            user: folder_to_scan.user.to_owned(),
        };

        if entry.file_type()?.is_dir() && folder_to_scan.sum_remaining_subfolders {
            tot += scan_folder_sum(&folder_inner)?.size;
        } else {
            tot += entry.metadata()?.len();
        }
    }

    Ok(FolderWithSize {
        folder: folder_to_scan.to_owned(),
        size: tot,
    })
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
    #[allow(dead_code)]
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
            log::trace!("scanning folder {:?}", folder);

            match folder.explode_depth {
                0 => v_sizes.push(scan_folder_sum(&folder)?),
                _ => v_sizes.extend_from_slice(&scan_folder_explode(&folder)?),
            }
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
		  		 { \"path\": \"pippo\", \"recurse_type\": \"Sum\", \"user\": \"pippo\" },
		  		 { \"path\": \"pluto\", \"recurse_type\": \"None\" , \"user\": \"pluto\"}, 
		  		 { \"path\": \"paperino\", \"recurse_type\": \"Explode\" },
		  		 { \"path\": \"other\", \"recurse_type\": \"None\" } 
		  	]
		  ";

        let dresp: FolderScanner = FolderScanner::from_json(s).unwrap();

        assert_eq!(dresp.folders().len(), 4);
        assert_eq!(dresp.folders()[0].recurse_type, RecurseType::Sum);
        assert_eq!(dresp.folders()[1].user, Some("pluto".to_owned()));
        assert_eq!(dresp.folders()[1].recurse_type, RecurseType::None);
        assert_eq!(dresp.folders()[2].path, "paperino");
        assert_eq!(dresp.folders()[2].user, None);
        assert_eq!(dresp.folders()[2].recurse_type, RecurseType::Explode);
        assert_eq!(dresp.folders()[3].recurse_type, RecurseType::None);
    }
}
