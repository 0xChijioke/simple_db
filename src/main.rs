use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);
    
    // unwrap is used to crash the program
    // expect(msg: "not found") can be used to show a more friendly msg
    //
    let key = args.next().unwrap();
    let value = args.next().unwrap();
    println!("the key is the argument you include while running the program {}", key);
    let cotents = format!("{}\t{}\n", key, value);
    let write_result = std::fs::write("kv.db", contents);
    match write_result {
        Ok(()) => {

        }
        Err(e) => {

        }
    
    }
    let database = Database::new();

}
    struct Database {
        map: HashMap<String, String>,
    }

    impl Database {
        fn new() -> Database {
            Database{
                map: HashMap::new(),
            }
        }
    }

