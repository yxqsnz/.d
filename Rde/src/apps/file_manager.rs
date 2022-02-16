use crate::popup;
use crate::{apps::text_editor, consts::FM_FPL};
use cursive::{
    views::{Dialog, DummyView, LinearLayout, RadioGroup},
    Cursive,
};
use std::fs;
use std::io::Result;
use std::path::PathBuf;

fn read_dir(path: impl Into<String>) -> Result<Vec<PathBuf>> {
    let mut entries = vec![];
    for entry in fs::read_dir(path.into())? {
        entries.push(entry?.path().to_path_buf());
    }
    Ok(entries)
}
fn get_entry_name(path: &PathBuf) -> String {
    if path.is_dir() {
        format!("{}/", path.file_name().unwrap().to_str().unwrap())
    } else {
        path.file_name().unwrap().to_str().unwrap().to_string()
    }
}
fn get_last_dir(path: &str) -> String {
    if path == "/" {
        path.to_string();
    }
    let pieces = path.split("/").collect::<Vec<&str>>();

    pieces[0..pieces.len() - 2].join("/").to_string()
}
pub fn get_window(display: &mut Cursive, read_path: String) {
    let mut file_group: RadioGroup<PathBuf> = RadioGroup::new();

    let files = match read_dir(&read_path) {
        Ok(entries) => entries,
        Err(e) => {
            popup!(display, "Can't read directories: {}", e);
            return;
        }
    };
    if files.is_empty() {
        popup!(display, title = "Files", "No files found.");
        return;
    }
    let prev = PathBuf::from(get_last_dir(files.first().unwrap().to_str().unwrap()));
    let mut entries = vec![prev.clone()];
    entries.extend(files);
    let chunks = entries.chunks(FM_FPL);
    let mut layout = LinearLayout::horizontal();
    for chunk in chunks {
        let mut line_files = LinearLayout::vertical();
        for entry in chunk {
            line_files.add_child(file_group.button(
                entry.to_owned(),
                if *entry == prev {
                    "previous".to_string()
                } else {
                    get_entry_name(entry)
                },
            ));
        }
        layout.add_child(line_files);
        layout.add_child(DummyView);
    }
    display.add_layer(
        Dialog::around(layout)
            .title(format!("Files - {}", read_path))
            .button("Open", move |s| {
                let entry_selected = file_group.selection();
                if entry_selected.is_dir() {
                    s.pop_layer();
                    get_window(s, entry_selected.to_string_lossy().to_string());
                } else {
                    text_editor::get_window(s, entry_selected.to_string_lossy().to_string());
                }
            }),
    );
}
