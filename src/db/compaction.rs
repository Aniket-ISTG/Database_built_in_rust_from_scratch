use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::io::Result;
use crate::db::constants::*;
use crate::db::engine::Database;

impl Database {
  pub fn compaction(&mut self) -> Result<()>{
    let mut compact_file = OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open("db.compact")?;

    let mut new_index:HashMap<String, u64> = HashMap::new();
    let mut new_offset = 0;
    
    for(key, _old_offset) in self.index.clone() {
      let value = self.get(&key)?.unwrap();

      compact_file.write_all(&[PUT_ENTRY])?;


      let key_bytes = key.as_bytes();
      let key_len = key_bytes.len() as u32;
      compact_file.write_all(&key_len.to_le_bytes())?;
      compact_file.write_all(key_bytes)?;

      let value_bytes  = value.as_bytes();
      let value_len = value_bytes.len() as u32;
      compact_file.write_all(&value_len.to_le_bytes())?;
      compact_file.write_all(value_bytes)?;

      new_index.insert(
        key.clone(),
        new_offset,
      );

      new_offset +=
      1 +
      4 + key_bytes.len() as u64 +
      4 + value_bytes.len() as u64;

      compact_file.sync_all()?;
    }
    drop(compact_file);

      std::fs::rename(
        "db.compact",
        "db.log"
      )?;

      self.file = OpenOptions::new()
      .read(true)
      .write(true)
      .open("db.log")?;

      self.index = new_index;
      self.current_offset = new_offset;
    Ok(())
  }

}