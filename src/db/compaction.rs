use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::io::Result;
use crate::db::constants::*;
use crate::db::engine::Database;
use crate::db::entry::Entry;
use crate::tree::btree::BTree;

impl Database {
  pub fn compaction(&mut self) -> Result<()>{
    let mut compact_file = OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open("db.compact")?;

    let mut new_index = BTree::new(3);
    let mut new_offset = 0;
    
    // traverse() returns entries in sorted order
    for (key, _old_offset) in self.index.traverse() {
      let value = self.get(&key)?.unwrap();
      let entry = Entry::new(PUT_ENTRY, key.clone(), Some(value.clone()));
      let entry_bytes = entry.serialize();
      compact_file.write_all(&entry_bytes)?;

      new_index.insert(
        key.clone(),
        new_offset,
      );

      new_offset +=
      1 +
      4 + key.as_bytes().len() as u64 +
      4 + value.as_bytes().len() as u64;

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