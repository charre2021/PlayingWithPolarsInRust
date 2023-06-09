use polars::prelude::*;
use std::fs;
use encoding::all::{ISO_8859_1, UTF_8};
use encoding::{Encoding, EncoderTrap, DecoderTrap};

fn main() 
{
    let read_file: String = String::from("metal.csv");
    let write_file : String = String::from("utf8encoded.csv");
    read_and_encode_file(&read_file, &write_file);
    let df = CsvReader::from_path(write_file)
    .unwrap()
    .with_null_values(Option::Some(NullValues::AllColumnsSingle(String::from("-"))))
    .finish()
    .unwrap();

    println!("{:?}", df);
}

fn read_and_encode_file(file_to_read : &String, file_to_write : &String) -> ()
{
    let byts_vec = fs::read(file_to_read).unwrap();
    let decoded = ISO_8859_1.decode(&byts_vec, DecoderTrap::Ignore).unwrap();
    let encoded = UTF_8.encode(&decoded, EncoderTrap::Ignore).unwrap();
    let empty_res = fs::write(file_to_write,encoded);
    
    if let Ok(()) = empty_res {
        println!("It's working! It's working!");
    } else {
        println!("AHHHHHHHHHH!");
    }
}