use chrono::{DateTime, Utc};
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::ops::DerefMut;
use std::path::PathBuf;
use std::sync::Mutex;
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;

pub struct ArchiveConfig {
    pub folder_with_files: PathBuf,
    pub archive_into_folder: PathBuf,
    pub files_before_date: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ArchiverProcessor {
    folder_with_files: PathBuf,
    list_of_files: Vec<PathBuf>,
    cutoff_date: DateTime<Utc>,
    archive_into_folder: PathBuf,
    resulting_zip_file: PathBuf,
}

trait Clone {
    fn clone(&self) -> Self;
}

impl Clone for ArchiverProcessor {
    fn clone(&self) -> Self {
        ArchiverProcessor {
            folder_with_files: self.folder_with_files.clone(),
            list_of_files: self.list_of_files.clone(),
            cutoff_date: self.cutoff_date,
            archive_into_folder: self.archive_into_folder.clone(),
            resulting_zip_file: self.resulting_zip_file.clone(),
        }
    }
}

impl ArchiverProcessor {
    pub fn new(config: ArchiveConfig, cutoff_date: DateTime<Utc>) -> ArchiverProcessor {
        let resulting_zip_file = config.folder_with_files.join("archive").join("archive.zip");

        ArchiverProcessor {
            folder_with_files: config.folder_with_files,
            list_of_files: Vec::new(),
            cutoff_date,
            archive_into_folder: config.archive_into_folder,
            resulting_zip_file,
        }
    }

    fn create_list_of_files(&mut self, folder_with_files: PathBuf) -> Vec<PathBuf> {
        for file in folder_with_files.read_dir().unwrap() {
            let file = file.unwrap();
            let file_path = file.path();
            self.list_of_files.push(file_path);
        }
        vec![self.list_of_files.swap_remove(0).clone()]
    }

    pub async fn archive_files(&mut self) {
        self.create_list_of_files(self.folder_with_files.clone());
        println!("Archiving files...");
        println!("List of files: {:#?}", self.list_of_files);
        println!();

        self.filter_files_before_cutoff();
        println!("List of files after filtering: {:#?}", self.list_of_files);
        // Need to move files to archive folder here then zip them
        self.move_files_to_archive_folder();
        println!();

        let list_of_files = self.list_of_files.clone();
        let archive_into_folder = self.archive_into_folder.clone();
        let resulting_zip_file = self.resulting_zip_file.clone();

        let _res: Result<(), io::Error> = async_std::task::spawn_blocking(move || {
            async_std::task::block_on(async {
                Self::zip_files_async(list_of_files, archive_into_folder, resulting_zip_file).await
            })
        })
        .await;
    }

    async fn zip_files_async(
        list_of_files: Vec<PathBuf>,
        archive_into_folder: PathBuf,
        resulting_zip_file: PathBuf,
    ) -> io::Result<()> {
        let file = File::create(resulting_zip_file)?;
        let file = BufWriter::new(file);

        let zip_mutex = Mutex::new(ZipWriter::new(file));
        let options = FileOptions::default().compression_method(CompressionMethod::Stored);

        list_of_files.par_iter().try_for_each(|path| {
            let relative_path = path
                .strip_prefix(&archive_into_folder)
                .unwrap_or(path)
                .to_string_lossy();
            println!("Archiving file: {:#?}", relative_path);

            if path.is_file() {
                let mut zip = zip_mutex.lock().unwrap();
                let mut zip_writer = zip.deref_mut();
                zip_writer.start_file(relative_path, options)?;
                let mut f = File::open(path)?;
                std::io::copy(&mut f, &mut zip_writer)?;
            }
            Ok::<(), std::io::Error>(())
        })?;

        let mut zip = zip_mutex.lock().unwrap();
        let zip_writer = zip.deref_mut();
        zip_writer.finish()?;

        Ok(())
    }

    fn filter_files_before_cutoff(&mut self) {
        self.list_of_files = self
            .list_of_files
            .par_iter()
            .filter_map(|file| {
                let file_modified = file.metadata().unwrap().modified().unwrap();
                let file_modified: DateTime<Utc> = DateTime::from(file_modified);
                if file_modified < self.cutoff_date {
                    Some(file.clone())
                } else {
                    None
                }
            })
            .collect();
    }

    fn move_files_to_archive_folder(&mut self) {
        let new_paths: Vec<PathBuf> = self
            .list_of_files
            .par_iter()
            .filter_map(|file_path| {
                if let Some(file_name) = file_path.file_name() {
                    let new_path = self.archive_into_folder.join(file_name);
                    match std::fs::rename(file_path, &new_path) {
                        Ok(_) => Some(new_path),
                        Err(err) => {
                            println!("Problem moving file: {}", err);
                            None
                        }
                    }
                } else {
                    eprintln!("Invalid file name for {}", file_path.display());
                    None
                }
            })
            .collect();

        println!("New paths: {:#?}", new_paths);
        self.list_of_files = new_paths;
    }

    fn dump_list_to_csv(&self, output_dir: PathBuf) -> io::Result<()> {
        // Extract the original directory name
        let dir_name = self
            .folder_with_files
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();

        // Generate a timestamp
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();

        // Construct the CSV file name
        let file_name = format!("{}_{}.csv", dir_name, timestamp);
        let output_file = output_dir.join(file_name);

        let mut file = File::create(output_file)?;

        // Writing the header (optional)
        writeln!(file, "File Path")?;

        for path in &self.list_of_files {
            let file_path = path.to_string_lossy();
            writeln!(file, "{}", file_path)?;
        }

        println!("List can be found at: {}", output_dir.display());

        Ok(())
    }

    pub fn request_dump_list(&self) {
        println!("Would you like to dump the list of files to a CSV document? (y/n)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().eq_ignore_ascii_case("y") {
            println!("Do you want to dump the list of files to a specific directory? (y/n)");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let output_dir = if input.trim().eq_ignore_ascii_case("y") {
                println!("Enter the directory path:");
                input.clear();
                io::stdin()
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

            match self.dump_list_to_csv(output_dir) {
                Ok(_) => println!("List successfully dumped to CSV."),
                Err(e) => eprintln!("Failed to dump list: {}", e),
            }
        }
    }
}
