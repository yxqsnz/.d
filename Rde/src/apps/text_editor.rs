use crate::{catch, popup};
use cursive::traits::Nameable;
use cursive::views::{Dialog, LinearLayout, TextArea};
use cursive::Cursive;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
pub fn get_window(display: &mut Cursive, filename: String) {
    catch!({
        if !Path::new(&filename).exists() {
            File::create(&filename)?;
        }
    } then(err) => {
        display.pop_layer();
        popup!(display, "Can't create file: {}", err);
        return
    });
    catch!({
        let mut file = OpenOptions::new().read(true).open(&filename).unwrap();

        let mut content = String::new();

        file.read_to_string(&mut content)?;

        let input_area = TextArea::new().content(content.as_str()).with_name("input");

        display.add_layer(
            Dialog::new()
                .title(&filename)
                .content(LinearLayout::horizontal().child(input_area))
                .button("Exit", |s| {
                    s.pop_layer();
                })
                .button("Save and Exit", move |s| {
                    catch!({
                    let mut file = OpenOptions::new()
                        .write(true)
                        .open(filename.as_str())?;
                    let content = s
                        .call_on_name("input", |d: &mut TextArea| d.get_content().to_string()).unwrap_or_default();

                        file.write_all(content.as_bytes())?;
                    } then(err) => {
                        popup!(s, title = "Editor",  "Can't save file: {}", err);
                        return
                    });
                    s.pop_layer();
                }),
        );
    } then(err) => {
        display.pop_layer();
        popup!(display, title = "Editor", "Error: {}", err);
        return
    });
}
