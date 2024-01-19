// pub mod arch_config {
//     use chrono::{DateTime, Utc};
//     use std::path::PathBuf;
//     pub struct ArchiveConfig {
//         pub folder_with_files: PathBuf,
//         pub archive_into_folder: PathBuf,
//         pub files_before_date: DateTime<Utc>,
//     }
// }

/*










*/

pub mod archive_processor {
    use crate::interface::interface_builder::ValidArgs;
    // use crate::structs::config::Config;

    // use super::arch_config::ArchiveConfig;
    use chrono::{DateTime, Utc};
    use rayon::prelude::*;
    use std::fs::File;
    use std::io::{self, BufWriter};
    use std::path::PathBuf;
    use std::sync::Mutex;
    use tokio::task;
    use zip::{write::FileOptions, CompressionMethod, ZipWriter};

    #[derive(Debug)]
    pub struct ArchiverProcessorAsync {
        folder_with_files: PathBuf,
        archive_into_folder: PathBuf,
        files_before_date: DateTime<Utc>,
    }

    // Overview call functions
    impl ArchiverProcessorAsync {
        pub fn new(config: ValidArgs) -> ArchiverProcessorAsync {
            ArchiverProcessorAsync {
                folder_with_files: config.src_path,
                archive_into_folder: config.archive_name,
                files_before_date: config.archive_all_before,
            }
        }

        pub async fn process_archiving(
            &self,
            archive_into_folder: std::path::PathBuf,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let files_to_archive = self.filter_files_before_date().await?;
            let moved_files = self.move_files_to_archive_folder(files_to_archive).await?;
            self.zip_files_async(moved_files, archive_into_folder)
                .await?;
            Ok(())
        }
    }

    // Filtering
    impl ArchiverProcessorAsync {
        async fn filter_files_before_date(&self) -> Result<Vec<PathBuf>, io::Error> {
            let mut files_to_archive = Vec::new();
            let mut entries = tokio::fs::read_dir(&self.folder_with_files).await?;

            // Debug
            // if entries.next_entry().await?.is_none() {
            //     return Ok(files_to_archive);
            // }

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                if self.is_file_eligible(&path).await? {
                    files_to_archive.push(path);
                }
            }
            let files_to_archive = files_to_archive
                .par_iter()
                .filter(|path| path.is_file())
                .cloned()
                .collect();

            Ok(files_to_archive)
        }

        async fn is_file_eligible(&self, path: &PathBuf) -> Result<bool, std::io::Error> {
            let metadata = tokio::fs::metadata(path).await?;
            let modified = metadata.modified().unwrap();
            let modified: DateTime<Utc> = DateTime::from(modified);

            println!("File: {:#?} Modified: {:#?}", path, modified);
            // dbg!(modified < self.files_before_date);
            let validation = Ok(modified < self.files_before_date);
            validation
        }
    }

    // Moving files
    impl ArchiverProcessorAsync {
        pub async fn move_files_to_archive_folder(
            &self,
            files: Vec<PathBuf>,
        ) -> Result<Vec<PathBuf>, io::Error> {
            let mut moved_files = Vec::new();
            for file in files {
                let new_path = self.archive_into_folder.join(file.file_name().unwrap());
                tokio::fs::rename(&file, &new_path).await?;
                moved_files.push(new_path);
            }
            Ok(moved_files)
        }
    }

    //Zipping of files
    impl ArchiverProcessorAsync {
        pub async fn zip_files_async(
            &self,
            files: Vec<PathBuf>,
            archive_into_folder: PathBuf,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let zip_file_path = self.archive_into_folder.join("archive.zip");
            let file = File::create(&zip_file_path)?;
            let writer = BufWriter::new(file);

            let zip_mutex = Mutex::new(ZipWriter::new(writer));
            let options = FileOptions::default().compression_method(CompressionMethod::Stored);

            task::spawn_blocking(move || {
                files.par_iter().try_for_each(|path| {
                    let relative_path = path
                        .strip_prefix(&archive_into_folder)
                        .unwrap_or(path)
                        .to_string_lossy();

                    if path.is_file() {
                        let mut zip = zip_mutex.lock().unwrap();
                        zip.start_file(relative_path, options)?;
                        let mut f = File::open(path)?;
                        std::io::copy(&mut f, &mut *zip)?;
                    }
                    Ok::<(), std::io::Error>(())
                })?;

                let mut zip = zip_mutex.lock().unwrap();
                zip.finish()?;
                Ok::<(), std::io::Error>(())
            })
            .await??;
            Ok(())
        }
    }

    // Any Processing done here
    // impl ArchiverProcessorAsync {
    //     fn process_file_data_parallel(file_data: &[u8]) -> Vec<u8> {
    //         file_data
    //             .par_iter()
    //             .map(|&byte| {
    //                 todo!();
    //                 process_byte(*byte)
    //             })
    //             .collect();
    //     }

    //     fn process_byte(byte: u8) -> u8 {
    //         todo!()
    //     }
    // }
}

/*










*/

pub mod cli_helpers {
    use chrono::{DateTime, Utc};
    use std::path::PathBuf;

    pub fn parse_date(src: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        let naive = chrono::NaiveDateTime::parse_from_str(src, "%Y-%m-%d %H:%M:%S")?;
        let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);
        Ok(datetime)
    }

    pub struct CliQuestions {
        pub pause: bool,
        pub exit: bool,
    }

    impl CliQuestions {
        pub fn new() -> CliQuestions {
            CliQuestions {
                pause: false,
                exit: false,
            }
        }

        pub fn pause(&self) {
            println!("Press any key to continue...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
        }

        // pub fn ask_questions(&self) {
        //     Self::ask_questions(&self);
        // }

        fn qustion_dump_to_csv(&mut self) {
            println!("Would you like to dump the list of files to a CSV document? (y/n)");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if input.trim().eq_ignore_ascii_case("y") {
                println!("Do you want to dump the list of files to a specific directory? (y/n)");
                input.clear();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                let output_dir = if input.trim().eq_ignore_ascii_case("y") {
                    println!("Enter the directory path:");
                    input.clear();
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let trimmed_input = input.trim();
                    let unquoted_input =
                        if trimmed_input.starts_with('\"') && trimmed_input.ends_with('\"') {
                            &trimmed_input[1..trimmed_input.len() - 1]
                        } else {
                            trimmed_input
                        };

                    let os_string_type = PathBuf::into_os_string(PathBuf::from(unquoted_input));
                    println!("os_string_type is: {:#?}", os_string_type);
                    PathBuf::from(os_string_type)
                } else {
                    PathBuf::from(".") // Current directory
                };

                match self.write_out_csv(output_dir) {
                    Ok(_) => println!("List successfully dumped to CSV."),
                    Err(e) => eprintln!("Failed to dump list: {}", e),
                }
            }
        }

        fn write_out_csv(&self, output_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
            todo!()
        }
    }
}
