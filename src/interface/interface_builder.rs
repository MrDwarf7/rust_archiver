use chrono::{DateTime, Utc};
// use chrono::{DateTime, Utc};
use clap::{parser::RawValues, Arg, ArgAction, Command, ValueHint};
use emojis;
// use tokio::join;
use std::{fmt::Debug, io, fmt, path:: PathBuf};

use crate::processing::async_archiver::cli_helpers;
// use std::fmt;


#[derive(Debug)]
pub struct ValidArgs {
    pub src_path: PathBuf,
    pub archive_name: PathBuf,
    pub archive_all_before: DateTime<Utc>,
    pub flags : Vec<String>
}

impl ValidArgs {
    pub fn new(
        src_path: PathBuf,
        archive_name: PathBuf,
        archive_all_before: DateTime<Utc>,
        flags: Vec<String>
    ) -> ValidArgs {
        ValidArgs {
            src_path,
            archive_name,
            archive_all_before,
            flags
        }
    }
    pub fn print(&self) {
        println!("src_path: {:?}", self.src_path);
        println!("archive_name: {:?}", self.archive_name);
        println!("archive_all_before: {:?}", self.archive_all_before);
        println!("flags: {:?}", self.flags);
    }
        
}

pub struct InterfaceParser {
    pub args: Vec<String>,
}

impl InterfaceParser {
    pub fn new() -> InterfaceParser {
        InterfaceParser { args: Vec::new() }
    }

    pub fn setup_runtime(&self) -> ValidArgs {
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
            .about(format!("\t{rocket} Blazin' fast archiving, with Rust {crab}\n \t   Now adapted for network drive usage!"))
            .arg_required_else_help(true)
            
            // .after_long_help("After long help section when using --help")
            // before's
            // help is automatic
            // .after_help("This is a test")
            
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
            
            
            // First arg - source path
            .arg(
                Arg::new("src_path")
                // Base for arg call
                    .id("src_path")
                    .value_name("Source Path")
                    .index(1)
                    .required(true)
                    .display_order(1)
                    .value_parser(| val: &str | -> Result<PathBuf, io::Error> {
                        let path = PathBuf::from(val);
                        if path.exists() {
                            Ok(path)
                        } else {
                            Ok(path)
                        }
                    })
                    .value_parser(| val: &str | -> Result<PathBuf, io::Error> {
                        let path = PathBuf::from(val);
                        if path.exists() {
                            Ok(path)
                        } else {
                            Ok(path)
                        }
                    })
                    .action(ArgAction::Set)
                    
                // Help setup
                    .help_heading("Inputs")
                    .help(format!("<Required> - {open_folder} Sets the source path of the files to archive"))
                    .value_hint(ValueHint::DirPath)
                    .long_help(format!("{open_folder} <Required> \nProvide a source path to the folder containing the files you wish to archive, it can be provided with or without quotes.\n
(NOTE: If the path has spaces in it, it must be quited)."))
            )

           // Second arg - archive folder name 
            .arg(
                Arg::new("archive_name")
                // Base for arg call
                    .id("archive_name")
                    .value_name("Archive Folder Name")
                    .index(2)
                    .required(false)
                    .display_order(2)
                    .default_value("archive")
                    .default_missing_value("archive")
                    .value_parser(| val: &str | -> Result<PathBuf, io::Error> {
                        let path = PathBuf::from(val);
                        if path.exists() {
                            Ok(path)
                        } else {
                            Ok(path)
                        }
                    })
                    
                // Help setup
                    .help_heading("Inputs")
                    .help(format!("[Optional] - {folder_name} Name of folder to archive into"))
                    .long_help(format!("{folder_name} [Optional] \nProvide a source path to the folder containing the files you wish to archive, it can be provided with or without quotes.\n
(NOTE: If the path has spaces in it, it must be quited)."))
            )
            
            // Third arg - archive all before date
            .arg(
                Arg::new("archive_all_before")
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
                Arg::new("non_interactive_mode")
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
                    .action(ArgAction::SetTrue)
                // Help setup
                    .help_heading("Automation Flags")
                    .help(format!("[Automations] - {robot} Supplied in order to disable user interactions"))
                    .long_help(format!("\n{robot} [Automations] \nProvide the flag via the Command Line to disable all interactive prompts \n
Allows for the binary to be called from automatic services and prevents the need for user interaction.
Output of reports are no longer requested at runtime and instead automatically written to the same folder as the binary.\n
(NOTE: These dates are calculated in UTC time, meaning the local time may differ slightly."))
            )
            .get_matches();
        
        let src_path = matches.get_raw("src_path").unwrap();
        let archive_name = matches.get_raw("archive_name").unwrap();
        let archive_all_before = matches.get_raw("archive_all_before").unwrap();
        let non_interactive_mode = matches.get_raw("non_interactive_mode").unwrap();
        
        // let printable_args = Vec::from([
        //     &src_path,
        //     &archive_name,
        //     &archive_all_before,
        //     &non_interactive_mode
        // ]);
        
        let args_to_print = ArgsToPrint {
            src_path,
            archive_name,
            archive_all_before,
            non_interactive_mode: non_interactive_mode.clone(),
        };
        println!("Args to print: {:#?}", args_to_print);
        
       
        let src_path = &matches.get_one::<PathBuf>("src_path").expect("No source path provided").to_path_buf();
        let archive_name = matches.get_one::<PathBuf>("archive_name").expect("No archive name provided").clone();
        let archive_all_before_str = matches.get_one::<String>("archive_all_before").expect("No date provided");
        let archive_all_before = cli_helpers::parse_date(&archive_all_before_str).expect("Invalid date provided");

        self.printable_args();

        let valid_args_closure = || {
            let flags: Vec<String> = non_interactive_mode.into_iter()
            .map(|flag| flag.to_str().unwrap().to_owned()).collect::<Vec<String>>();
            // for flag in non_interactive_mode {
            //     flags.push(flag);
            // }
            let valid_args = ValidArgs::new(
                src_path.clone(),
                archive_name.clone(),
                archive_all_before,
                Vec::from(flags)
            );
            
            valid_args
        };

        // let valid_args: ValidArgs = valid_args_closure();

        valid_args_closure()
    }
    
    
    fn printable_args(&self) -> Vec<&String> {
        let src_path = self.args.get(1).unwrap();
        let archive_name = self.args.get(2).unwrap();
        let archive_all_before = self.args.get(3).unwrap();
        let non_interactive_mode = self.args.get(4).unwrap();
        
        let printable_args = Vec::from([
            src_path,
            archive_name,
            archive_all_before,
            non_interactive_mode
        ]);
        
        return printable_args
    }
}


pub struct EndProcessValidation {
    
}







 
#[derive(Debug)]
pub struct ArgsToPrint<'a> {
    src_path: RawValues<'a>,
    archive_name: RawValues<'a>,
    archive_all_before: RawValues<'a>,
    non_interactive_mode: RawValues<'a>,
}

impl<'a> fmt::Display for ArgsToPrint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "src_path: {:#?}\narchive_name: {:#?}\narchive_all_before: {:#?}\nnon_interactive_mode: {:#?}",
            self.src_path, self.archive_name, self.archive_all_before, self.non_interactive_mode)
    }
} 
   
   
    
    
    
    // fn parse_date(date_str: &str) -> Result<DateTime<Utc>, &'static str> {
    //     NaiveDate::parse_from_str(date_str, "%Y/%m/%d")
    //         .map_err(|_| "Invalid time. Date must be formatted as YYYY/MM/DD")
    //         .and_then(|date| date.and_hms_opt(0, 0, 0).ok_or("Invalid time"))
    //         .map(|naive_date| Utc.from_utc_datetime(&naive_date))
    // }
    
