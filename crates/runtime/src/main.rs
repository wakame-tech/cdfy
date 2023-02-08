use crate::gen::{bindings::Runtime, types::Data};

pub mod gen;

fn main() {
    let url = "https://gist.github.com/wakame-tech/50c25f0eecd3507f20396e6c21f51691/raw/0f45400319ecf50e2fdece815fc57d1379d4687e/example_plugin.wasm";
    let bytes = reqwest::blocking::get(url).unwrap().bytes().unwrap();
    let runtime = Runtime::new(bytes.as_ref()).unwrap();
    let r = runtime.data_check(Data {
        name: "xx".to_owned(),
        text: "yy".to_owned(),
    });
    println!("{r:?}");
    let r = runtime.data_check(Data {
        name: "".to_owned(),
        text: "aaa".to_owned(),
    });
    println!("{r:?}");
}
