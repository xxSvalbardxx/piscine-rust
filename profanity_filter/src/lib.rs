pub struct Message {
    content: String,
    //user : String,
}

impl Message {
  pub fn new(ms: String, _u: String) -> Message {
    Message {
        content: ms,
    //  user: u,
    }
  }

pub fn send_ms(&self) -> Option<&str> {
    if self.content.len() == 0 || self.content.contains("stupid") {
      return None;
    } else {
      return Some(&self.content);
    }
  }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        Some(_ms) => (true, ""),
        None => (false, "ERROR : illegal"),
    }
}

