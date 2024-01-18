use async_std::path::PathBuf;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use clap::{arg, command, value_parser, Arg, ArgAction, Command, Parser, ValueHint, ArgMatches};
use emojis::{self as emojis, Emoji};


struct ValidArgs {
    file_path: PathBuf,
    archive_folder_name: PathBuf,
    archive_all_before: DateTime<Utc>,
}

pub struct InterfaceParser {
    pub args: Vec<String>,
}

impl InterfaceParser {
    pub fn new() -> InterfaceParser {
        InterfaceParser { args: Vec::new() }
    }
    
    
    

    pub async fn setup_runtime(&self) {
        let rocket = emojis::get("🚀").unwrap();
        let crab = emojis::get("🦀").unwrap();
        let open_folder = emojis::get("📂").unwrap();
        let folder_name = emojis::get("📝").unwrap();
        let calendar = emojis::get("📅").unwrap();
        let robot = emojis::get("🤖").unwrap();
        
        // Base for command call
        let matches = Command::new("Rust Archiver")
            .version("0.1.0")
            .before_long_help("\n") // Placeholders for now
            .before_help("\n") // Placeholders for now
            .about(format!("\t{rocket} Blazin' fast archiving, with Rust {crab}.\n \t   Now adapted for network drive usage!"))
            .arg_required_else_help(true)
            // .after_long_help("After long help section when using --help")
            
            // Custom, non required About arg
            .arg(
                Arg::new("About")
                .short('a')
                .long("about")
                .hide_default_value(true)
                .hide_possible_values(true)
                .help(format!("More info can be found here: 
http://www.google.com --- Placeholder()
                "))
                .long_help("About the program")
            )
            
            
            // before's
            // help is automatic
            // .after_help("This is a test")
            
            // First arg - source path
            .arg(
                Arg::new("Source Path")
                // Base for arg call
                    .id("src_path")
                    .value_name("Source Path")
                    .index(1)
                    .required(true)
                    .display_order(1)
                // Help setup
                    .help_heading("Inputs")
                    .help(format!("<Required> - {open_folder} Sets the source path of the files to archive"))
                    .value_hint(ValueHint::DirPath)
                    .long_help(format!("{open_folder} <Required> \nProvide a source path to the folder containing the files you wish to archive, it can be provided with or without quotes.\n
(NOTE: If the path has spaces in it, it must be quited)."))
            )

           // Second arg - archive folder name 
            .arg(
                Arg::new("Archive Folder Name")
                // Base for arg call
                    .id("archive_name")
                    .value_name("Archive Folder Name")
                    .index(2)
                    .required(false)
                    .display_order(2)
                    .default_value("archive")
                    .default_missing_value("archive")
                // Help setup
                    .help_heading("Inputs")
                    .help(format!("[Optional] - {folder_name} Name of folder to archive into"))
                    .long_help(format!("{folder_name} [Optional] \nProvide a source path to the folder containing the files you wish to archive, it can be provided with or without quotes.\n
(NOTE: If the path has spaces in it, it must be quited)."))
            )
            
            // Third arg - archive all before date
            .arg(
                Arg::new("Archive All Before Date")
                // Base for arg call
                    .id("archive_all_before")
                    .value_name("Archive All Before Date")
                    .index(3)
                    .required(false)
                    .display_order(3)
                    .default_value("2023/01/01")
                    .default_missing_value("2023/01/01")
                // Help setup
                    .help_heading("Inputs")
                    .help(format!("[Optional] - {calendar} Archive all files before before the given date. Format: YYYY/MM/DD"))
                    .long_help(format!("{calendar} [Optional] \nFormat: YYYY/MM/DD \nProvide a date from which all files before/prior to the given date are archived.\nUses the Date Last Modified metadata to perform the checks.\n
(NOTE: These dates are calculated in UTC time, meaning the local time may differ slightly."))
            )
            
            .arg(
                Arg::new("Non-Interactive Mode")
                // Base for arg call
                    .id("non_interactive_mode")
                    .value_name("Non-Interactive Mode")
                    .short_aliases(['N', 'n'])
                    .long("non-interactive")
                    // .visible_short_aliases(['N', 'n'])
                    .visible_aliases([
                        "N",
                        "n",
                        "non-interactive",
                    ])
                    // .index(4)
                    .required(false)
                    .display_order(4)
                    .default_value(None)
                // Help setup
                    .help_heading("Automation Flags")
                    .help(format!("[Automations] - {robot} Supplied in order to disable user interactions"))
                    .long_help(format!("\n{robot} [Automations] \nProvide the flag via the Command Line to disable all interactive prompts \n
Allows for the binary to be called from automatic services and prevents the need for user interaction.
Output of reports are no longer requested at runtime and instead automatically written to the same folder as the binary.\n
(NOTE: These dates are calculated in UTC time, meaning the local time may differ slightly."))
            )
            .get_matches();
        
        let src_path: Option<&ArgMatches> = matches.try_get_one("Source Path").unwrap();
        let archive_name: Option<&ArgMatches> = matches.try_get_one("Archive Folder Name").unwrap();
        let archive_all_before: Option<&ArgMatches> = matches.try_get_one("Archive All Before Date").unwrap();
        let non_interactive_mode: Option<&ArgMatches> = matches.try_get_one("Non-Interactive Mode").unwrap();
        
        
        
        println!("src_path: {:?}", src_path);
        println!("archive_name: {:?}", archive_name);
        println!("archive_all_before: {:?}", archive_all_before);
        println!("non_interactive_mode: {:?}", non_interactive_mode);
        
        
        
        
        
        
        
        
    }
}
