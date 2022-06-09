use ferris_says::say;
use std::io::{stdout, BufWriter};

pub fn log(s: &[u8]) {
    let width = 32;
    let mut writer = BufWriter::new(stdout());
    say(s, width, &mut writer).unwrap();
}
