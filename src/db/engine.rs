use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::io::Result;
use crate::db::constants::*;
use crate::db::entry::Entry;
use crate::db::recovery::load_index;
use crate::tree::btree::BTree;


pub struct Database{
  pub file : File,
  pub index : BTree,
  pub current_offset : u64,
}


impl Database {

  pub fn open(path : &str) -> Result<Self> {

    let mut file = OpenOptions::new().create(true).read(true).write(true).open(path)?;

    let mut index = BTree::new(3);

    let current_offset = load_index(&mut file, &mut index)?;

    return Ok(Self{
      file,
      index,
      current_offset
    });
  }

  pub fn put(&mut self, key : &str, val : &str) -> Result<()>{
    let key_bytes = key.as_bytes();
    let val_bytes = val.as_bytes();
    self.file.seek(SeekFrom::Start(self.current_offset))?;

    let entry  = Entry::new(PUT_ENTRY, key.to_string(), Some(val.to_string()));
    let serialized_entry = entry.serialize();
    self.file.write_all(&serialized_entry)?;

    ////////////////////////// Force to write it on disk
    self.file.sync_all()?;

    self.index.insert(key.to_string(), self.current_offset);

    ///////////////////////////////////////////////////// update next free offset
    self.current_offset += 1 +
        4 + key_bytes.len() as u64 +
        4 + val_bytes.len() as u64;

    Ok(())
  }

  pub fn get(&mut self, key : &str) -> Result<Option<String>> {

    let offset_opt = self.index.get(key);
    match offset_opt {
      Some(offset) => {
        ////////////////////////////////////////////// Jump to the entry
        self.file.seek(SeekFrom::Start(offset))?;

        ///////////////////////////////////////////// Type of the Entry
        let mut type_buf = [0u8 ; 1];
        self.file.read_exact(&mut type_buf)?;
        let entry_type = type_buf[0];
        if entry_type != PUT_ENTRY {
          return Ok(None);
        } 

        ///////////////////////////////////////////// Read key len
        let mut key_len_buf = [0u8; 4];
        self.file.read_exact(&mut key_len_buf)?;
        let key_len = u32::from_le_bytes(key_len_buf) as u32;

        ///////////////////////////////////////////// Read key data/buf
        let mut key_buf = vec![0u8 ; key_len as usize];
        self.file.read_exact(&mut key_buf)?;
        
        ///////////////////////////////////////////// Read val len
        let mut val_len_buf = [0u8; 4];
        self.file.read_exact(&mut val_len_buf)?;
        let val_len = u32::from_le_bytes(val_len_buf) as u32;

        //////////////////////////////////////////// Read val data/buf
        let mut value_buf = vec![0u8 ; val_len as usize];
        self.file.read_exact(&mut value_buf)?;
        let value = String::from_utf8(value_buf).unwrap();

        return Ok(Some(value));
      },
      None => return Ok(None),
    }
  }
  pub fn delete(&mut self, key : &str) -> Result<()> {
    
    let key_bytes = key.as_bytes();
    self.file.seek(SeekFrom::Start(self.current_offset))?;

    let entry = Entry::new(DELETE_ENTRY, key.to_string(), None);
    let serialized_entry = entry.serialize();
    self.file.write_all(&serialized_entry)?;


    self.file.sync_all()?;
    self.index.remove(key);
    self.current_offset +=
    1 +
    4 +
    key_bytes.len() as u64 +
    4; // and value length field(0)
    Ok(())
  }
  
}