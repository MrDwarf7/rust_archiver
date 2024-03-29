use std::time::Instant;
use tokio::main;

// use core_deps::{interface::InterfaceParser, processing::ArchiverProcessorAsync};

use core_deps::{interface::InterfaceParser, processing::ArchiverProcessorAsync};

#[main]
async fn main() {
    let start = Instant::now();

    let interface_parser = InterfaceParser::new();
    let valid_args = interface_parser.setup_runtime();

    println!("Valid args: {:#?}", valid_args);

    // startregion

    // todo!("Finalize implmentatiing CLAP as the interface parser.");
    //
    //
    // let args: Vec<String> = std::env::args().collect();

    // let config = Config::new(valid_args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {}", err);
    //     std::process::exit(1);
    // });

    // println!("Config is: {:#?}", config);

    // let arch_config = ArchiveConfig {
    //     folder_with_files: config.folder_with_files.clone(),
    //     archive_into_folder: config.archive_into_folder.clone(),
    //     files_before_date: config.files_before_date,
    // };

    // endregion

    // let mut archiver = ArchiverProcessor::new(arch_config, config.files_before_date);
    let async_archiver = ArchiverProcessorAsync::new(valid_args);

    todo!();

    if let Err(e) = async_archiver.process_archiving(valid_args.src_path).await {
        eprintln!("Error during arching process: {}", e);
    }

    let duration = start.elapsed();
    println!("Time taken to complete run: {:?}", duration);

    // async_archiver.request_dump_list();
    // let questions = CliQuestions::new();
    // questions.pause();
    // questions.ask_questions();
    //
}
