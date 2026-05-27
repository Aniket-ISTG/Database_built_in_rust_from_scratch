use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::io::Result;


const PUT_ENTRY: u8 = 1;
const DELETE_ENTRY: u8 = 2;
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

  fn put(&mut self, key : &str, val : &str) -> Result<()>{
    let key_bytes = key.as_bytes();
    let val_bytes = val.as_bytes();
    let key_len = key_bytes.len() as u32;
    let val_len = val_bytes.len() as u32;
    self.file.seek(SeekFrom::Start(self.current_offset))?;

    self.file.write_all(&[PUT_ENTRY])?;

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
    self.current_offset += 1 +
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

  fn delete(&mut self, key : &str) -> Result<()> {
    
    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len() as u32;
    self.file.seek(SeekFrom::Start(self.current_offset))?;
    self.file.write_all(&[DELETE_ENTRY])?;
    self.file.write_all(&key_len.to_le_bytes())?;
    self.file.write_all(key_bytes)?;
    self.file.sync_all()?;
    self.index.remove(key);
    self.current_offset +=
    1 +
    4 +
    key_bytes.len() as u64;
    Ok(())
  }

  fn compaction(&mut self) -> Result<()>{
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

fn main() -> Result<()> {

    // -----------------------------------
    // REMOVE OLD DATABASE FOR CLEAN TEST
    // -----------------------------------
    let _ = std::fs::remove_file("db.log");
    let _ = std::fs::remove_file("db.compact");

    // -----------------------------------
    // OPEN DATABASE
    // -----------------------------------
    let mut db = Database::open("db.log")?;

    // -----------------------------------
    // PUT TESTS
    // -----------------------------------
    println!("\n========== PUT TESTS ==========\n");

    db.put("name", "Aniket")?;
    db.put("age", "19")?;
    db.put("city", "Bhopal")?;

    println!("Inserted:");
    println!("name -> Aniket");
    println!("age  -> 19");
    println!("city -> Bhopal");

    // -----------------------------------
    // GET TESTS
    // -----------------------------------
    println!("\n========== GET TESTS ==========\n");

    println!("name => {:?}", db.get("name")?);
    println!("age  => {:?}", db.get("age")?);
    println!("city => {:?}", db.get("city")?);

    // -----------------------------------
    // UPDATE TEST
    // -----------------------------------
    println!("\n========== UPDATE TEST ==========\n");

    db.put("name", "Sen")?;

    println!("Updated:");
    println!("name -> Sen");

    println!("name => {:?}", db.get("name")?);

    // -----------------------------------
    // DELETE TEST
    // -----------------------------------
    println!("\n========== DELETE TEST ==========\n");

    db.delete("age")?;

    println!("Deleted:");
    println!("age");

    println!("age => {:?}", db.get("age")?);

    // -----------------------------------
    // SHOW INDEX BEFORE COMPACTION
    // -----------------------------------
    println!("\n========== INDEX BEFORE COMPACTION ==========\n");

    println!("{:#?}", db.index);

    // -----------------------------------
    // FILE SIZE BEFORE COMPACTION
    // -----------------------------------
    let before_size =
        std::fs::metadata("db.log")?.len();

    println!(
        "db.log size before compaction: {} bytes",
        before_size
    );

    // -----------------------------------
    // COMPACTION TEST
    // -----------------------------------
    println!("\n========== COMPACTION ==========\n");

    db.compaction()?;

    println!("Compaction completed!");

    // -----------------------------------
    // FILE SIZE AFTER COMPACTION
    // -----------------------------------
    let after_size =
        std::fs::metadata("db.log")?.len();

    println!(
        "db.log size after compaction: {} bytes",
        after_size
    );

    // -----------------------------------
    // VERIFY DATA AFTER COMPACTION
    // -----------------------------------
    println!("\n========== VERIFY AFTER COMPACTION ==========\n");

    println!("name => {:?}", db.get("name")?);
    println!("age  => {:?}", db.get("age")?);
    println!("city => {:?}", db.get("city")?);

    // -----------------------------------
    // RESTART RECOVERY TEST
    // -----------------------------------
    println!("\n========== RESTART RECOVERY TEST ==========\n");

    drop(db);

    let mut recovered_db =
        Database::open("db.log")?;

    println!("Recovered values:");

    println!(
        "name => {:?}",
        recovered_db.get("name")?
    );

    println!(
        "age => {:?}",
        recovered_db.get("age")?
    );

    println!(
        "city => {:?}",
        recovered_db.get("city")?
    );

    // -----------------------------------
    // FINAL INDEX
    // -----------------------------------
    println!("\n========== FINAL INDEX ==========\n");

    println!("{:#?}", recovered_db.index);

    Ok(())
}