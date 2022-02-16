use cursive::{
    event::Event::CtrlChar,
    CursiveRunnable,
};

use super::apps;

pub struct WindowManager {
    display: CursiveRunnable,
}

impl WindowManager {
    pub fn new(display: CursiveRunnable) -> WindowManager {
        WindowManager { display: display }
    }
    pub fn init(&mut self) {
        self.display.add_global_callback(CtrlChar('d'), |s| {
            apps::applauncher::get_window(s);
        });
        self.display.add_global_callback(CtrlChar('w'), |s| {
            s.pop_layer();
        });

        self.display.run();
    }
}
