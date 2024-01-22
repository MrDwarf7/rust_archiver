use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use chrono::{DateTime, NaiveDate, TimeZone, Utc};

use crate::interface::interface_builder::ValidArgs;

#[derive(Debug)]
// #[allow(dead_code)]
pub struct Config {
    pub folder_with_files: PathBuf,
    pub archive_into_folder: PathBuf,
    pub files_before_date: DateTime<Utc>,
}

impl Config {
    pub fn new(args: ValidArgs) -> Result<Config, &'static str> {
        // if args.len() < 2 {
        //     return Err("Not enough arguments");
        // }

        // let folder_with_files = PathBuf::from(&args[1]);
        let archive_into_folder = Self::determine_archive_folder(&folder_with_files, &args); // Fix: Pass &self as the first argument
        let files_before_date = Self::determine_cutoff_date(&args)?;

        println!("Folder with files: {:?}", folder_with_files);
        println!("Archive into folder: {:?}", archive_into_folder);
        println!("Files before date: {:?}", files_before_date);

        let archive_into_folder = match archive_into_folder {
            Ok(folder) => folder.to_path_buf(),
            Err(err) => {
                println!("Problem creating archive folder: {}", err);
                std::process::exit(1);
            }
        };

        Ok(Config {
            folder_with_files,
            archive_into_folder,
            files_before_date,
        })
    }

    fn determine_archive_folder(
        folder_with_files: &Path,
        args: &[String],
    ) -> Result<PathBuf, &'static str> {
        let folder_name = if args.len() > 2 && !args[2].is_empty() {
            &args[2]
        } else {
            "archive"
        };
        Config::create_archive_folder(folder_with_files, folder_name)
    }

    fn create_archive_folder(base_path: &Path, folder_name: &str) -> Result<PathBuf, &'static str> {
        let archive_folder = base_path.join(folder_name);
        if !archive_folder.exists() {
            std::fs::create_dir_all(&archive_folder).map_err(|_| "Failed to create directory")?;
        }
        Ok(archive_folder)
    }

    fn determine_cutoff_date(args: &[String]) -> Result<DateTime<Utc>, &'static str> {
        if args.len() > 3 && !args[3].is_empty() {
            Self::parse_date(&args[3])
        } else {
            Self::parse_date("2023/01/01") // Default date
        }
    }

    pub fn parse_date(date_str: &str) -> Result<DateTime<Utc>, &'static str> {
        NaiveDate::parse_from_str(date_str, "%Y/%m/%d")
            .map_err(|_| "Invalid time. Date must be formatted as YYYY/MM/DD")
            .and_then(|date| date.and_hms_opt(0, 0, 0).ok_or("Invalid time"))
            .map(|naive_date| Utc.from_utc_datetime(&naive_date))
    }
}
