extern crate colored;
use colored::*;

mod math;
mod load_obj;
use load_obj::*;

use std::fs::File;
use std::io::{Write, BufWriter};
use std::mem;
use std::slice;

fn main() {
    let path = ask_path();
    let load_data_order = ask_data_order();
    let data = ObjLoader::new().load(path, load_data_order);
    write_data_to_file(data);
}

fn write_data_to_file(data : Vec<f32>){
    let file = File::create("output.vbo").unwrap();
    let mut writer = BufWriter::new(file);
    let raw_data = &data[..];
    let raw_data_u8: &[u8] = unsafe {
        slice::from_raw_parts(
            raw_data.as_ptr() as *const u8, raw_data.len() * mem::size_of::<f32>()
        )
    };

    writer.write_all(raw_data_u8).unwrap();
}

fn ask_data_order() -> Vec<BufferDataType>{
    println!("select output data order (example : [{}]):", "tu tv tw vx vy * nz *".green());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let data_order_string = input[0..input.len() - 2].to_string();
    let mut data_order : Vec<BufferDataType> = Vec::new();
    for data_descr_string in data_order_string.split_whitespace(){
        let data_descr = match data_descr_string{
            "vx" => { BufferDataType::PosX }
            "vy" => { BufferDataType::PosY }
            "vz" => { BufferDataType::PosZ }
            
            "nx" => { BufferDataType::NormX }
            "ny" => { BufferDataType::NormY }
            "nz" => { BufferDataType::NormZ }

            "tu" => { BufferDataType::TexU }
            "tv" => { BufferDataType::TexV }
            "tw" => { BufferDataType::TexW }

            _ => { BufferDataType::Void }
        };
        data_order.push(data_descr);
    }
    data_order
}

fn ask_path() -> String {
    println!("{}", "enter obj file name :".green());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input[0..input.len() - 2].to_string()
}