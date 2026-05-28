mod db;
mod tree;
mod query_parser;

use db::engine::Database;
use query_parser::parser::parse;
use query_parser::actions::Command;
fn main() -> Result<(), Box<dyn std::error::Error>> {

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



    println!("Now run your own queries using the query parser!");
    println!("Example:");
    println!("> INSERT name Aniket");
    println!("> GET name");
    println!("> DELETE name"); 
    loop {
        println!("\nEnter a command (INSERT, GET, DELETE) or 'exit' to quit:");
        std::io::Write::flush(&mut std::io::stdout())?;
        
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input)?;

        match parse(&input) {
            Some(cmd) => {
                if matches!(cmd, Command::Exit) {
                    println!("Exiting...");
                    break;
                }
                query_parser::execute::execute(cmd, &mut recovered_db);
            },
            None => eprintln!("Invalid command. Please try again."),
        }
    }

    Ok(())
}