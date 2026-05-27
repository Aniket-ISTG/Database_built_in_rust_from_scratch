use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::io::Result;
use crate::db::constants::*;

  pub fn load_index(file : &mut File, index : &mut HashMap<String, u64>) -> Result<u64>{
    let mut offset = 0;
    file.seek(SeekFrom::Start(0))?;

    loop{
      // for now format is :
      // for index.put("xyz", someOffsetNumberOrAddress)
      // now that someOffsetNumberOrAddress ----points_to-----> actual data in the disk 
      // [keyLength][key][valueLength][value]

      /////////////////////////////////////////// Read the type of entry
      let mut type_buf = [0u8; 1];
      if let Err(_) = file.read_exact(&mut type_buf) {
          break;
      }

      let entry_type = type_buf[0];

      //////////////////////////////////////////// Reading key length
      let mut key_len_buf = [0u8; 4];

      // IS IT THE END OF THE FILE  
      if let Err(_) = file.read_exact(&mut key_len_buf) {
        break;  
      }


      let key_len = u32::from_le_bytes(key_len_buf) as u32;


      //////////////////////////////////////////// Reading key
      let mut key_buf = vec![0u8; key_len as usize];
      file.read_exact(&mut key_buf)?;
      let key = String::from_utf8(key_buf).unwrap();


      if entry_type == PUT_ENTRY {

        let mut val_len_buf = [0u8; 4];
        file.read_exact(&mut val_len_buf)?;

        let val_length =
            u32::from_le_bytes(val_len_buf) as u32;

        let mut value_buf =
            vec![0u8; val_length as usize];

        file.read_exact(&mut value_buf)?;

        index.insert(key, offset);

        offset +=
            1 +
            4 + key_len as u64 +
            4 + val_length as u64;

      } else if entry_type == DELETE_ENTRY {

          index.remove(&key);

          offset +=
              1 +
              4 + key_len as u64;
        }

    }
    
    return Ok(offset);

  }