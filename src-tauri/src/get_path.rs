/*use tauri::api::dialog::FileDialogBuilder;

#[tauri::command]
fn open_file_dialog() -> Option<String> {
    // Open the file dialog to select a file
    let file_path = FileDialogBuilder::new().pick_file().ok().and_then(|path| path.to_str().map(String::from));

    file_path
}*/