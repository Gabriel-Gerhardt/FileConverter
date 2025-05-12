// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod file_convert;
use file_convert::pages_to_pdf;
mod hello;
use hello::greet;
fn main() {
    let input_path = "/Users/gabrielgerhardt/Downloads/GabrielGerhardtDeMarque_autoria.pages";
    let output_path = "/Users/gabrielgerhardt/documents/arquivos_pdf";

    if let Err(e) = pages_to_pdf(input_path, output_path) {
        eprintln!("Failed to convert .pages file to PDF: {}", e);
    }
    //fileconverter_lib::run()
}
