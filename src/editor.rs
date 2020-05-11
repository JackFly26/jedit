use pancurses::{Input, Window, getmouse};
use std::fs::File;
use std::io::Read;

pub struct Editor<'a> {
    window: &'a Window,
    buffer: String,
    file: &'a File,
    index: usize,
    starting_line: usize,
    starting_column: usize
}

impl<'a> Editor<'a> {
    pub fn new(window: &'a Window, file: &'a mut File) -> std::io::Result<Self> {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        window.addstr(&buffer);
        window.mv(0, 0);
        Ok(Editor {
            window: window,
            file: file,
            buffer: buffer,
            index: 0,
            starting_line: 0,
            starting_column: 0
        })
    }
    fn mv(&mut self, x: i32, y: i32) {
        use std::cmp::min;
        let mut lines: Vec<&str> = self.buffer.lines().collect();
        let mut y = min(y + self.starting_line as i32, lines.len() as i32 - 1);
        if y < 0 { y = 0 }
        let mut x = x + self.starting_column as i32;
        if x < 0 {
            if y > 0 {
                y -= 1;    
                x += lines[y as usize].len() as i32 + 1;
                if x < 0 {
                    x = 0;
                }    
            } else {
                x = 0;
            }
        } else if x > lines[y as usize].len() as i32 {
            if y < lines.len() as i32 - 1 {
                y += 1;
                x -= lines[y as usize - 1].len() as i32 + 1;
                self.window.addstr(format!("({}, {})", x, y));
                if x < 0 || x > lines[y as usize].len() as i32 {
                    x = 0;
                }
            } else {
                x = lines[y as usize].len() as i32;
            }
        }
        let x = x as usize;
        let y = y as usize;
        let _ = lines.split_off(y);
        let rest = lines.into_iter().fold(0, |acc, next| acc + next.len() + 1);
        self.index = rest + x;
        self.window.mv((y - self.starting_line) as i32, (x - self.starting_column) as i32);
    }
    fn delch(&mut self) {
        let x = self.window.get_cur_x();
        let y = self.window.get_cur_y();
        let _ = self.buffer.remove(self.index);
        self.window.mv(0, 0);
        self.window.clear();
        self.window.addstr(&self.buffer);
        self.window.mv(y, x);
    }
    fn addch(&mut self, c: char) {
        let x = self.window.get_cur_x();
        let y = self.window.get_cur_y();
        self.buffer.insert(self.index, c);
        self.window.mv(0, 0);
        self.window.clear();
        self.window.addstr(&self.buffer);
        self.mv(x + 1, y);
    }
    pub fn run(&mut self) {
        loop {
            match self.window.getch() {
                Some(inp) if [Input::KeyUp, Input::KeyDown, Input::KeyLeft, Input::KeyRight].contains(&inp) => {
                    let mut x = self.window.get_cur_x();
                    let mut y = self.window.get_cur_y();
                    match inp {
                        Input::KeyUp => y -= 1,
                        Input::KeyDown => y += 1,
                        Input::KeyLeft => x -= 1,
                        Input::KeyRight => x += 1,
                        _ => panic!("match statements are broken")
                    }
                    self.mv(x, y);
                }
                Some(Input::KeyBackspace) => {
                    let x = self.window.get_cur_x();
                    let y = self.window.get_cur_y();
                    self.mv(x - 1, y);
                    self.delch();
                }
                Some(Input::KeyMouse) => {
                    if let Ok(mouse_event) = getmouse() {
                        let lines: Vec<&str> = self.buffer.lines().collect();
                        let x = std::cmp::min(mouse_event.x, lines.get(mouse_event.y as usize).unwrap_or(&lines[lines.len() - 1]).len() as i32);
                        self.mv(x, mouse_event.y);
                    }
                }
                Some(Input::Character(c)) if c == '\x1b' => {
                    if let Some(inp) = self.window.getch() {

                    } else {
                        break;
                    }
                }
                Some(Input::Character(c)) => {
                    self.addch(c);
                }
                Some(input) => {
                    self.window.addstr(&format!("{:?}", input));
                }
                None => (),
            }
            if self.window.is_touched() {
                self.window.refresh();
            }
        }
    }
}
