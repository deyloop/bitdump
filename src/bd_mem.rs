use libbitdump::BitDumpKV;

#[cfg(target_os = "windows")]
const USAGE: &str = "
    bd_mem.exe FILE get KEY
    bd_mem.exe FILE delete KEY
    bd_mem.exe FILE insert KEY VALUE
    bd_mem.exe FILE update KEY VALUE  
";


#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
    bd_mem FILE get KEY
    bd_mem FILE delete KEY
    bd_mem FILE insert KEY VALUE
    bd_mem FILE update KEY VALUE  
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);

    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = BitDumpKV::open(path).expect("Unable to open file");
    store.load().expect("unable to load data");

    match action {
        "get" => match store.get(key).unwrap() {
            None => eprintln!("{:?} not found", key),
            Some(value) => println!("{:?}", value),
        },
        "delete" => store.delete(key).unwrap(),
        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap()
        }
        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap()
        }
        _ => eprintln!("{}", &USAGE),
    }
}
