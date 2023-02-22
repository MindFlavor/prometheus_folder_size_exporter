use crate::folder_scanner::FolderScanner;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
pub(crate) struct Options {
    pub verbose: bool,
    pub folders_to_scan: FolderScanner,
    pub background_poll_seconds: Option<std::time::Duration>,
}

impl Options {
    pub fn from_claps(matches: &clap::ArgMatches<'_>) -> Options {
        let folders_file = matches.value_of("folders_file").unwrap().to_owned();
        let background_poll_seconds = matches
            .value_of("background_poll_seconds")
            .map(|s| std::time::Duration::from_secs(s.parse().unwrap()));

        let mut file = File::open(folders_file).unwrap();
        let mut file_contents = String::new();

        file.read_to_string(&mut file_contents).unwrap();

        Options {
            verbose: matches.is_present("verbose"),
            folders_to_scan: FolderScanner::from_json(&file_contents).unwrap(),
            background_poll_seconds,
        }
    }
}
