mod data;
use std::io::{self, Write};
use serde_json::Value;
use rand::Rng;

fn rand_name() -> io::Result<String> {
    let v: Value = serde_json::from_str(data::DATA)?;
    let list = v.as_array().unwrap();
    let num = rand::thread_rng().gen_range(0..list.len());
    Ok(list[num].to_string().replace("\"", ""))
}

fn main() {
    match rand_name() {
        Ok(name) => {
            let _ = writeln!(&mut std::io::stdout(), "{}", name);
        },
        Err(err) => {
            let _ = writeln!(&mut std::io::stderr(), "{}", err);
        }
    }
}