#![feature(core)]

extern crate "rustc-serialize" as rustc_serialize;
#[macro_use] #[no_link] extern crate mopa;

use rustc_serialize::json;

use demo::Delta;

mod demo;

fn main() {
    let mut doc = demo::DocumentWithHistory::new();

    doc.apply(demo::DeltaSetName::new("new name".to_string()));
    doc.apply(demo::DeltaAppendContents::new("some contents".to_string()));

    let encoded = json::encode(&doc).unwrap();
    println!("encoded = {}", encoded);

    let decoded: demo::DocumentWithHistory = json::decode(&encoded).unwrap();
    let reencoded = json::encode(&decoded).unwrap();

    println!("reencoding matches? {}", reencoded == encoded);
}
