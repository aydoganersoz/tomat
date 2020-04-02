use indicatif::ProgressBar;

pub struct Bar {
  pb: ProgressBar,
}

impl Bar {
  pub fn new(len: u32) -> Bar {
    Bar {
      pb: ProgressBar::new(len as u64),
    }
  }

  pub fn increase(&self) {
    self.pb.inc(1);
  }
}
