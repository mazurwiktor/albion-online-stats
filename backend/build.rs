// build.rs

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("itemdb.rs");
    let mut f = File::create(&dest_path).unwrap();

    let mut out = String::new();

    out.push_str("use std::collections::HashMap;\n");
    out.push_str("lazy_static! {\n");
    out.push_str("pub static ref ITEMDB: HashMap<u32, &'static str> = {[\n");

    include_str!("assets/item_ids.txt").split('\n').filter_map(|line| {
        let v: Vec<&str> = line.split(',').collect();
        let id : u32 = v.get(0)?.parse().ok()?;
        let item  = v.get(1)?.to_owned();
        Some((id, item))
    }).for_each(|(id, item)| {
        out.push_str(&format!("({}, \"{}\"),\n", id, item))
    });
    out.push_str("].iter().cloned().collect()\n");
    out.push_str("};\n");
    out.push_str("}");

    f.write_all(out.as_bytes()).unwrap();
}
