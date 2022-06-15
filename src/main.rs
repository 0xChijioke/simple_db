use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);
    
    // unwrap is used to crash the program
    // expect(msg: "not found") can be used to show a more friendly msg
    //
    let key = args.next().unwrap();
    let value = args.next().unwrap();
    println!("the key is the argument you include while running the program {}", key);
    let contents = format!("{}\t{}\n", key, value);
    let write_result = std::fs::write("kv.db", contents);
    // match write_result {
    //     Ok(()) => {

    //     }
    //     Err() => {

    //    should go back to rusty times }
    
    // }
    let database = Database::new();

}
    struct Database {
        map: HashMap<String, String>,
    }

    impl Database {
        fn new() -> Result<Database, std::io::Error> {
            // read the kv.db file
            // let contents = match std::fs::read_to_string("kv.db"){
               // Ok(c) => c,
               // Err(error) => {
               //     return Err(error);
               // }
           // };
            let contents = std::fs::read_to_string("kv.db")?;
            for line in contents.lines() {
                let pair = line.split_once('\t').expect("currupt");
            }
           // parse the string

            // populate our map
            Ok(Database{
                map: HashMap::new(),
            })
        }
    }

