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
