use libbitdump::BitDumpKV;
use std::collections::HashMap;

#[cfg(target_os = "windows")]
const USAGE: &str = "
    bd_disk.exe FILE get KEY
    bd_disk.exe FILE delete KEY
    bd_disk.exe FILE insert KEY VALUE
    bd_disk.exe FILE update KEY VALUE  
";


#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
    bd_disk FILE get KEY
    bd_disk FILE delete KEY
    bd_disk FILE insert KEY VALUE
    bd_disk FILE update KEY VALUE  
";

use libbitdump::{ByteStr,ByteString};

fn store_index_on_disk(a: &mut BitDumpKV, index_key: &ByteStr) {
    a.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&a.index).unwrap();
    a.index = std::collections::HashMap::new();
    a.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
    const INDEX_KEY: &ByteStr = b"+index";

    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);

    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = BitDumpKV::open(path).expect("Unable to open file");
    store.load().expect("unable to load data");


    match action {
        "get" => {
            let index_as_bytes = store
                .get(&INDEX_KEY)
                .expect("Could not load index").unwrap();

            let index_decoded = bincode::deserialize(&index_as_bytes);

            let index: HashMap<ByteString, u64> = index_decoded.unwrap();

            match index.get(key) {
                None => eprintln!("{:?} not found", key),
                Some(&i) => {
                    let kv = store.get_at(i).unwrap();
                    println!("{:?}", kv.value)
                },
            }
        }        

        "delete" => store.delete(key).unwrap(),

        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap();
            store_index_on_disk(&mut store, INDEX_KEY);
        }

        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap();
            store_index_on_disk(&mut store, INDEX_KEY);
        }
        _ => eprintln!("{}", &USAGE),
    }
}
