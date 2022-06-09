use ferris_says::say;
use std::io::{stdout, BufWriter};

pub fn log(s: &str) {
    let width = 32;
    let mut writer = BufWriter::new(stdout());
    say(s.as_bytes(), width, &mut writer).unwrap();
}
