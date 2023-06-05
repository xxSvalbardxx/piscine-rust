pub struct Message {
    content: String,
    user : String,
}

impl Message {
  pub fn new(ms: String, u: String) -> Message {
    Message { ms, u }
  }
  pub fn send_ms(&self) -> Option<&str> {
    if self.content.len() == 0 || self.content == "stupid" {
      None
    } else {
      Some(&self.content)
    }
  }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        Some(ms) => (true, ms),
        None => (false, "ERROR : illegal"),
    }

}