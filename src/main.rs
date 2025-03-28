use std::{fs::File, io::Write, process::Command};
use zerocopy::IntoBytes;

fn main() {
    let buf: Vec<u8> = wave()
        .iter()
        .flat_map(|sample| sample.as_bytes())
        .map(|b| *b)
        .collect();

    // TODO: proper error handling

    let mut file = File::create("output.bin").unwrap();
    file.write_all(&buf).unwrap();

    println!("wrote sound to output.bin");
    println!("playing sound");

    // ffplay -f f32le -ar 48000 output.bin
    Command::new("ffplay")
        .arg("-f")
        .arg("f32le")
        .arg("-ar")
        .arg("48000")
        .arg("output.bin")
        .output()
        .unwrap();
}

fn wave() -> Vec<f32> {
    let max = 48000;
    let seconds = 1;
    let samples = 0..max * seconds;
    let step = 0.05;
    let volume = 0.5;

    samples
        .map(|sample| (sample as f32 * step * volume).sin())
        .collect()
}
