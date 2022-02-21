use grid::{Grid, GridMode};
use pancurses::{
    curs_set, endwin, has_colors, init_pair, initscr, napms, noecho, raw, start_color, Input,
    Window,
};
use std::{env, process::exit};
mod grid;
mod pixel;
use easy_args::arg_spec;
use pixel::Color;
use std::path::Path;

fn init() -> Window {
    let mut window = initscr();
    window.keypad(true);
    noecho();
    raw();
    window.nodelay(true);

    /*INIT COLORS*/
    if !has_colors() {
        println!("YOUR TERMINAL DOESNT SUPPORT COLORS!");
        exit(0);
    }
    start_color();
    curs_set(0);
    init_pair(1, 0, 0); //black
    init_pair(2, 1, 1); //red
    init_pair(3, 2, 2); //green
    init_pair(4, 3, 3); //yellow
    init_pair(5, 4, 4); //blue
    init_pair(6, 5, 5); //magenta
    init_pair(7, 6, 6); //cyan
    init_pair(8, 7, 7); //white
    init_pair(9, 0, 7);
    return window;
}

fn main() {
    let args = arg_spec! {
        height: u64,
        width: u64,
        filename: String
    };

    //let mut grid = Grid::new(args);
    let args = if let Ok(a) = args.parse() {
        a
    } else {
        println!("tifeditor help\n\n --filename <filename>\n    sets the filename if the file exists the editor will try to load the image from that file\n    if not, it'll create a new file and save the image with this name\n\n --height <number>\n    sets the height of the image\n\n --width <number>\n    sets the width of the image\n");
        exit(1);
    };

    let mut grid = {
        if let Some(filename) = args.string("filename") {
            if Path::new(filename).exists() {
                Grid::load_from_file(filename.to_string())
            } else {
                let height = *args.uinteger("height").unwrap_or(&30) as i16;
                let width = *args.uinteger("width").unwrap_or(&30) as i16;
                Grid::new((height, width))
            }
        } else {
            let height = *args.uinteger("height").unwrap_or(&30) as i16;
            let width = *args.uinteger("width").unwrap_or(&30) as i16;
            Grid::new((height, width))
        }
    };

    let window = init();
    if grid.height as i32 + 7 > window.get_max_y()
        || (grid.width as i32 > window.get_max_x() && grid.width > 16)
    {
        endwin();
        println!("YOUR TERMINAL IS TOO SMALL!");

        return;
    }

    'mainloop: loop {
        if let Some(ch) = window.getch() {
            match ch {
                Input::KeyLeft => grid.dec_select_width(),
                Input::KeyRight => grid.inc_select_width(),
                Input::KeyUp => grid.dec_select_height(),
                Input::KeyDown => grid.inc_select_height(),
                Input::Character(c) => {
                    if c == 's' || c == 'S' {
                        if let Some(filename) = args.string("filename") {
                            grid.save_to_file(filename.to_string());
                        } else {
                            grid.save_to_file("image.tif".to_string());
                        }
                    } else if c == 'i' {
                        grid.mode = GridMode::Editing;
                    } else if c == '\x1b' {
                        grid.mode = GridMode::Selection;
                    } else if c == ' ' && grid.mode == GridMode::Editing {
                        grid.set_selected_color_in_pixel(&window);
                    } else if c == 'w' || c == 'W' {
                        grid.dec_select_height();
                        if grid.mode == GridMode::Editing {
                            grid.set_selected_color_in_pixel(&window);
                        }
                    } else if c == 's' || c == 'S' {
                        grid.inc_select_height();
                        if grid.mode == GridMode::Editing {
                            grid.set_selected_color_in_pixel(&window);
                        }
                    } else if c == 'a' || c == 'D' {
                        grid.dec_select_width();
                        if grid.mode == GridMode::Editing {
                            grid.set_selected_color_in_pixel(&window);
                        }
                    } else if c == 'd' || c == 'D' {
                        grid.inc_select_width();
                        if grid.mode == GridMode::Editing {
                            grid.set_selected_color_in_pixel(&window);
                        }
                    }
                    if let Ok(num) = c.to_string().parse::<u8>() {
                        if grid.mode == GridMode::Selection {
                            let new_color = match num {
                                1 => Color::Black,
                                2 => Color::Red,
                                3 => Color::Green,
                                4 => Color::Yellow,
                                5 => Color::Blue,
                                6 => Color::Magenta,
                                7 => Color::Cyan,
                                8 => Color::White,
                                _ => grid.selected_color,
                            };
                            grid.selected_color = new_color;
                        }
                    }
                }
                _ => {}
            }
        }
        grid.draw(&window);
        grid.draw_colors(&window);
        grid.draw_selected_color(&window);
        grid.draw_current_mode(&window);
        //napms(20);
    }

    endwin();
}
