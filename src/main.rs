mod editor;

use pancurses::{endwin, initscr, mousemask, noecho, ALL_MOUSE_EVENTS};
use editor::Editor;
use std::env;
use std::path::Path;
use std::fs::File;

fn main() -> std::io::Result<()>{
    let env: Vec<String> = env::args().collect();
    let filename = &env[1];
    let path = Path::new(&filename);
    let mut file = File::open(path)?;
    let window = initscr();
    window.refresh();
    window.keypad(true);
    mousemask(ALL_MOUSE_EVENTS, std::ptr::null_mut());
    noecho();
    window.nodelay(true);
    let mut editor = Editor::new(&window, &mut file)?;
    editor.run();
    endwin();
    Ok(())
}
