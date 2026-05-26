use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::io::Result;

struct Database{
  file : File,
  index : HashMap<String, u64>,
  current_offset : u64,
}


impl Database{
  fn open(path : &str) -> Result<Self> {

    let mut file = OpenOptions::new().create(true).read(true).write(true).open(path)?;

    let mut index = HashMap::new();

    let current_offset = Self::load_index(&mut file, &mut index)?;

    return Ok(Self{
      file,
      index,
      current_offset
    });
  }

  fn load_index(file : &mut File, index : &mut HashMap<String, u64>) -> Result<u64>{
    let mut offset = 0;
    file.seek(SeekFrom::Start(0))?;

    loop{
      // for now format is :
      // for index.put("xyz", someOffsetNumberOrAddress)
      // now that someOffsetNumberOrAddress ----points_to-----> actual data in the disk 
      // [keyLength][key][valueLength][value]

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


      //////////////////////////////////////////// Reading value length
      let mut val_len_buf = [0u8; 4];
      file.read_exact(&mut val_len_buf)?;
      let val_length = u32::from_le_bytes(val_len_buf) as u32;

      //////////////////////////////////////////// Skiping value
      let mut value_buf = vec![0u8; val_length as usize];
      file.read_exact(&mut value_buf)?;

      // latest offset wins
      index.insert(key, offset);

      offset += 4 + key_len as u64 + 4 +  val_length as u64;
    }
    
    return Ok(offset);

  }

  fn put(&mut self, key : &str, val : &str) -> Result<()>{
    let key_bytes = key.as_bytes();
    let val_bytes = val.as_bytes();
    let key_len = key_bytes.len() as u32;
    let val_len = val_bytes.len() as u32;
    self.file.seek(SeekFrom::Start(self.current_offset))?;

    /////////////////////////////////////////////////////// Write key length
    self.file.write_all(&key_len.to_le_bytes())?;


    ////////////////////////////////////////////////////// Write key data/buf
    self.file.write_all(key_bytes)?;

    ////////////////////////////////////////////////////// Write val length
    self.file.write_all(&val_len.to_le_bytes())?;

    ///////////////////////////////////////////////////// Write val data/buf
    self.file.write_all(val_bytes)?;


    ////////////////////////// Force to write it on disk
    self.file.sync_all()?;

    self.index.insert(key.to_string(), self.current_offset);

    ///////////////////////////////////////////////////// update next free offset
    self.current_offset +=
        4 + key_bytes.len() as u64 +
        4 + val_bytes.len() as u64;

    Ok(())
  }

  fn get(&mut self, key : &str) -> Result<Option<String>> {

    let does_key_exist_enum = self.index.get(key);
    match does_key_exist_enum {
      Some(v) => {
        let offset = *v;

        ////////////////////////////////////////////// Jump to the entry
        self.file.seek(SeekFrom::Start(offset))?;

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
}

fn main() -> Result<()> {

    let mut db =
        Database::open("db.log")?;

    db.put("name", "Aniket")?;
    db.put("age", "19")?;
    db.put("name", "Sen")?;

    println!("{:?}", db.get("name")?);
    println!("{:?}", db.get("age")?);

    Ok(())
}