mod interface;
mod processing;
mod structs;
use interface::InterfaceParser;
use processing::{ArchiveConfig, ArchiverProcessorAsync, CliQuestions};
use std::time::Instant;
use structs::config::Config;

// Add args[0] that indicates it's being run as an automated process
// ie: via python or something, and shifts args by 1 (Can hold args in a struct and utilize a method handler for this )
//

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let interface_parser = InterfaceParser::new();

    let valid_args = interface_parser.setup_runtime();

    println!("Valid args: {:#?}", &valid_args);

    todo!("Finalize implmentatiing CLAP as the interface parser.");
    //
    //
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("Config is: {:#?}", config);

    let arch_config = ArchiveConfig {
        folder_with_files: config.folder_with_files.clone(),
        archive_into_folder: config.archive_into_folder.clone(),
        files_before_date: config.files_before_date,
    };

    // let mut archiver = ArchiverProcessor::new(arch_config, config.files_before_date);
    let async_archiver = ArchiverProcessorAsync::new(arch_config, config.files_before_date);

    if let Err(e) = async_archiver
        .process_archiving(config.archive_into_folder)
        .await
    {
        eprintln!("Error during arching process: {}", e);
    }

    let duration = start.elapsed();
    println!("Time taken to complete run: {:?}", duration);

    // async_archiver.request_dump_list();
    let questions = CliQuestions::new();

    questions.pause();
    questions.ask_questions();
    //
}
