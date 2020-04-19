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
        let mut x = min(x + self.starting_line as i32, lines[y as usize].len() as i32);
        if x < 0 {
            y -= 1;
            if y < 0 {
                y = 0;
                x = 0;
            } else {
                x = lines[y as usize].len() as i32 + x;
            }
            if x < 0 {
                x = 0;
            }
        }
        let x = x as usize;
        let y = y as usize;
        let _ = lines.split_off(y);
        let rest = lines.into_iter().fold(0, |acc, next| acc + next.len() + 1);
        self.index = rest + x;
        self.window.mv((y - self.starting_line) as i32, (x - self.starting_column) as i32);
        //self.window.addstr(&format!("{}", self.index));
    }
    fn delch(&mut self) {
        let x = self.window.get_cur_x();
        let y = self.window.get_cur_y();
        let removed = self.buffer.remove(self.index);
        if removed.is_whitespace() {
            self.window.delch();
        }
        self.window.mv(0, 0);
        self.window.addstr(&self.buffer);
        self.window.mv(y, x);
    }
    fn addch(&mut self, c: char) {
        self.window.insch(c);
        let x = self.window.get_cur_x();
        let y = self.window.get_cur_y();
        self.buffer.insert(self.index, c);
        self.window.mv(0, 0);
        self.window.addstr(&self.buffer);
        self.mv(x + 1, y);
    }
    pub fn run(&mut self) {
        loop {
            match self.window.getch() {
                Some(Input::KeyBackspace) => {
                    let x = self.window.get_cur_x();
                    let y = self.window.get_cur_y();
                    self.mv(x - 1, y);
                    self.delch();
                }
                Some(Input::KeyMouse) => {
                    if let Ok(mouse_event) = getmouse() {
                        self.mv(mouse_event.x, mouse_event.y);
                    }
                }
                Some(Input::Character(c)) if c == 'q' => {
                    break;
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
