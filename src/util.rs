use std::fs;
use std::process;

pub fn create_dir() -> std::io::Result<()> {
  fs::create_dir_all("db")?;

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
