use std::process::Command;
use std::fs;
use std::io;

pub fn pages_to_pdf(input_path: &str, output_path: &str) -> Result<(), io::Error> {
    // Ensure input file exists
    if !fs::metadata(input_path).is_ok() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Input file not found"));
    }

    // Check if LibreOffice is installed by running 'libreoffice --version'
    let libreoffice_check = Command::new("soffice")
        .arg("--version")
        .output();

    match libreoffice_check {
        Ok(output) if !output.stdout.is_empty() => {
            println!("LibreOffice found: {}", String::from_utf8_lossy(&output.stdout));
        }
        _ => {
            return Err(io::Error::new(io::ErrorKind::NotFound, "LibreOffice not found. Please install it."));
        }
    }

    // Run LibreOffice to convert the .pages file to PDF
    let status = Command::new("soffice")
        .arg("--headless")  // Run without GUI
        .arg("--convert-to")
        .arg("pdf")
        .arg(input_path)
        .arg("--outdir")
        .arg(output_path)
        .status()?;

    if status.success() {
        println!("Conversion successful!");
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Conversion failed"))
    }
}


