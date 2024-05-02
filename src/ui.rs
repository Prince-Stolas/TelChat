use std::io::{LineWriter, Write};

pub fn draw_input_box<T>(writer: &mut LineWriter<T>, width: i32, contents: String) where T: ?Sized + Write {
    let _ = writer.write("-".repeat(width as usize).as_bytes());
    let _ = writer.write(&[b'\n']);

    let _ = writer.write(&[b'|']);
    let _ = writer.write(contents
        .clone()
        .self_truncate(width - 2)
        .expand(width - 2)
        .as_bytes());
    let _ = writer.write(&[b'|']);
    let _ = writer.write(&[b'\n']);

    let _ = writer.write("-".repeat(width as usize).as_bytes());
    let _ = writer.write(&[b'\n']);
}

pub trait StringUtils {
    fn expand(self, len: i32) -> Self;

    fn self_truncate(self, len: i32) -> Self;
}

impl StringUtils for String {
    fn expand(mut self, len: i32) -> Self {
        if len < self.len() as i32 {
            return self;
        }
        let spaces_amt = len as usize - self.len();
        self.push_str(&*" ".repeat(spaces_amt));
        self
    }

    fn self_truncate(mut self, len: i32) -> Self {
        self.truncate(len as usize);
        self
    }
}