pub struct Entry {
  pub entry_type : u8,
  pub key : String,
  pub value : Option<String>,
}

impl Entry {
  pub fn new(entry_type : u8, key : String, value : Option<String>) -> Self {
    Entry {
      entry_type,
      key,
      value,
    }
  }
  pub fn serialize(&self) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(self.entry_type);

    let key_bytes = self.key.as_bytes();
    let key_len = key_bytes.len() as u32;
    buf.extend_from_slice(&key_len.to_le_bytes());
    buf.extend_from_slice(key_bytes);

    if let Some(val) = &self.value {
      let val_bytes = val.as_bytes();
      let val_len = val_bytes.len() as u32;
      buf.extend_from_slice(&val_len.to_le_bytes());
      buf.extend_from_slice(val_bytes);
    } else {
      buf.extend_from_slice(&0u32.to_le_bytes());
    }
    buf
  }

  pub fn deserialize(mut buf : &[u8]) -> Self {
    let entry_type = buf[0];
    buf = &buf[1..];

    let key_len = u32::from_le_bytes(buf[0..4].try_into().unwrap()) as usize;
    buf = &buf[4..];

    let key = String::from_utf8(buf[0..key_len].to_vec()).unwrap();
    buf = &buf[key_len..];

    let val_len = u32::from_le_bytes(buf[0..4].try_into().unwrap()) as usize;
    buf = &buf[4..];

    let value = if val_len > 0 {
      Some(String::from_utf8(buf[0..val_len].to_vec()).unwrap())
    } else {
      None
    };

    Entry {
      entry_type,
      key,
      value,
    }
  }
}