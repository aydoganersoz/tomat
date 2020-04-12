const EXECUTABLE_PATH: &str = "./target/debug/tomat";

#[test]
fn no_cli_argument() {
  /*let output = std::process::Command::new(EXECUTABLE_PATH)
  .arg("--help")
  .output()
  .expect("failed to execute process");*/
  
  try to use this for the integration testing
  get_matches_from_safe

  assert!(main().is_err());
  // println!("{:?}", output);
  //assert_eq!(String::from_utf8_lossy(&output.stderr), "ss");
}
