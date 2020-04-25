use rodio::Source;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::process;

pub fn create_dir() -> std::io::Result<()> {
  fs::create_dir_all("db")?;
  fs::create_dir_all("out")?;

  Ok(())
}

pub fn exit_program() {
  // TODO: implement exit codes
  process::exit(0);
}

pub fn register_sigint() {
  ctrlc::set_handler(move || on_sigint()).expect("register sigint failed");
}

fn on_sigint() {
  println!("closing tomat...");
  exit_program();
}

pub fn play_bip() {
  let device = rodio::default_output_device().unwrap();

  let file = File::open("rsc/A-Tone-His_Self-1266414414.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
  rodio::play_raw(&device, source.convert_samples());
}
