use rustc_serialize::{Encoder, Encodable, Decoder, Decodable};
use std::any::Any;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Document {
    pub name: String,
    pub contents: String,
}

impl Document {
    pub fn new() -> Document {
        Document{
            name: "".to_string(),
            contents: "".to_string(),
        }
    }
}

#[derive(RustcEncodable, RustcDecodable)]
enum DeltaKindTag {
    SetName,
    AppendContents,
}

pub trait Delta: Any {
    fn apply(&self, &mut Document);
}

mopafy!(Delta);

impl Encodable for Box<Delta> {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        if let Some(delta) = self.downcast_ref::<DeltaSetName>() {
            (DeltaKindTag::SetName, delta).encode(s)
        } else if let Some(delta) = self.downcast_ref::<DeltaAppendContents>() {
            (DeltaKindTag::AppendContents, delta).encode(s)
        } else {
            panic!("Unknown concrete delta type")
        }
    }
}

impl Decodable for Box<Delta> {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_tuple(2, |d| {
            let tag = try!(d.read_tuple_arg(0, |d| Decodable::decode(d)));
            d.read_tuple_arg(1, |d| {
                let boxed = match tag {
                    DeltaKindTag::SetName =>
                        Box::new(try!(DeltaSetName::decode(d))) as Box<Delta>,
                    DeltaKindTag::AppendContents =>
                        Box::new(try!(DeltaAppendContents::decode(d))) as Box<Delta>,
                };
                Ok(boxed)
            })
        })
    }
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct DeltaSetName {
    name: String,
}

impl DeltaSetName {
    pub fn new(name: String) -> DeltaSetName {
        DeltaSetName{ name: name }
    }
}

impl Delta for DeltaSetName {
    fn apply(&self, doc: &mut Document) {
        doc.name = self.name.clone();
    }
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct DeltaAppendContents {
    contents: String,
}

impl DeltaAppendContents {
    pub fn new(contents: String) -> DeltaAppendContents {
        DeltaAppendContents{ contents: contents }
    }
}

impl Delta for DeltaAppendContents {
    fn apply(&self, doc: &mut Document) {
        doc.contents.push_str(&self.contents);
    }
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct DocumentWithHistory {
    doc: Document,
    history: Vec<Box<Delta>>,
}

impl DocumentWithHistory {
    pub fn new() -> DocumentWithHistory {
        DocumentWithHistory{
            doc: Document::new(),
            history: Vec::new(),
        }
    }

    //pub fn apply<T: Delta>(&mut self, delta: T) {
    pub fn apply<T: Delta + 'static>(&mut self, delta: T) {
        delta.apply(&mut self.doc);
        self.history.push(Box::new(delta));
    }
}
