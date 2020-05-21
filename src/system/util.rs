use crate::system::string;
use chrono::Utc;
use dialoguer::Confirmation;
use rodio::Source;
use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::PathBuf;
use std::{process, thread, time};

pub fn print_terminal(arg: &str) {
  println!("{}", arg);
}

pub fn exit_program() {
  thread::spawn(|| {
    thread::sleep(time::Duration::new(5, 0));
    process::abort();
  });
  process::exit(0);
}

pub fn register_sigint() -> Result<(), ctrlc::Error> {
  ctrlc::set_handler(move || {
    exit_program();
  })?;
  Ok(())
}

pub fn play_sound(path: PathBuf) {
  let file = File::open(path);
  match file {
    Ok(file) => {
      let h = thread::spawn(move || {
        let device = rodio::default_output_device().unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        rodio::play_raw(&device, source.convert_samples());
      });
      let _result = h.join();
    }
    Err(_e) => print_terminal(string::ERR_READ_SOUND_FILE),
  }
}

pub fn construct_filename(fields: Vec<&str>) -> String {
  let mut filename = String::new();
  for field in fields {
    filename.push_str(field);
  }
  filename
}

pub fn construct_path(fields: Vec<&str>, extension: &str) -> PathBuf {
  let mut path = PathBuf::new();
  for field in fields {
    path.push(field);
  }
  path.set_extension(extension);
  path
}

pub fn get_timestamp() -> i64 {
  Utc::now().timestamp()
}

pub fn create_file(path: PathBuf) -> io::Result<File> {
  let file = File::create(path)?;
  Ok(file)
}

pub fn create_directory(path_string: &str) -> io::Result<()> {
  let path = PathBuf::from(path_string);
  fs::create_dir_all(path)?;
  Ok(())
}

pub fn yes_no_confirmation(text: &str) -> Result<bool, io::Error> {
  let result = Confirmation::new().with_text(&text).interact()?;
  Ok(result)
}
