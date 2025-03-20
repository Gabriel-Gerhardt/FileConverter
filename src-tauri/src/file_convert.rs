use docx_rs::Docx;
use printpdf::*;

fn docs_to_pdf() {
    let docx_data = std::fs::read(" input.docx").expect("Failed to read DOCX");
    let docx = Docx::new(docx_data.as_slice()).expect("Failed to parse DOCX");

    let mut pdf = Pdf::new();
    let mut page = pdf.add_page(PageSize::A4);

    for paragraph in docx.paragraphs {
        let text = paragraph.text.unwrap_or_default();
        page.add_text(text.as_str());
    }

    pdf.save("output.pdf").expect("Failed to save PDF");
}
