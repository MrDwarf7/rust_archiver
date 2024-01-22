pub mod helpder_structs {

    use chrono::{DateTime, Utc};
    use std::path::PathBuf;

    #[derive(Debug)]
    pub struct ValidArgs {
        pub src_path: PathBuf,
        pub archive_name: PathBuf,
        pub archive_all_before: DateTime<Utc>,
        pub flags: Vec<String>,
    }

    impl ValidArgs {
        pub fn new(
            src_path: PathBuf,
            archive_name: PathBuf,
            archive_all_before: DateTime<Utc>,
            flags: Vec<String>,
        ) -> ValidArgs {
            ValidArgs {
                src_path,
                archive_name,
                archive_all_before,
                flags,
            }
        }
        pub fn print(&self) {
            println!("src_path: {:?}", self.src_path);
            println!("archive_name: {:?}", self.archive_name);
            println!("archive_all_before: {:?}", self.archive_all_before);
            println!("flags: {:?}", self.flags);
        }
    }
}
