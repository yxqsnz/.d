use super::{file_manager, text_editor};
use crate::popup;
use cursive::{
    traits::{Boxable, Nameable},
    views::{Dialog, EditView, LinearLayout},
    Cursive,
};
use std::env;
use std::path::Path;


pub fn get_window(display: &mut Cursive) {
    display.add_layer(
        Dialog::new()
            .title("App Launcher")
            .content(
                LinearLayout::horizontal().child(EditView::new().with_name("app").fixed_width(50)),
            )
            .button("launch", |s| {
                let app = s
                    .call_on_name("app", |d: &mut EditView| d.get_content().to_string())
                    .unwrap();
                s.pop_layer();

                let pieces: Vec<&str> =
                    app.as_str().split_ascii_whitespace().collect::<Vec<&str>>();
                let appname = pieces[0];

                match appname {
                    "editor" => {
                        if pieces.len() < 2 {
                            popup!(s, "Please provide a file: editor file.txt");
                        } else {
                            let file = pieces[1];
                            text_editor::get_window(s, file.to_string());
                        }
                    }
                    "files" => {
                        let mut directory =
                            String::from(env::current_dir().unwrap().to_str().unwrap());
                        if pieces.len() > 1 {
                            let f = Path::new(pieces[1]);
                            if f.exists() && f.is_dir() {
                                directory = String::from(pieces[1]);
                            } else {
                                popup!(s, "Path {} doesn't exist or isn't a directory", pieces[1]);
                                return;
                            }
                        }
                        file_manager::get_window(s, directory);
                    }
                    _ => {
                        popup!(s, "Unrecognized {}", app);
                    }
                };
            }),
    )
}
