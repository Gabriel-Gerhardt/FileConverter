// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod file_convert;
use file_convert::pages_to_pdf; 
fn main() {
    let input_path = "/Users/gabrielgerhardt/downloads/prob_TDE1.pages";
    let output_path = "/Users/gabrielgerhardt/documents/test";

    if let Err(e) = pages_to_pdf(input_path, output_path) {
        eprintln!("Failed to convert .pages file to PDF: {}", e);
    //fileconverter_lib::run()
}
}