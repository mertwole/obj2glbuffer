use crate::math::*;

use colored::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub enum BufferDataType{
    PosX,
    PosY,
    PosZ,

    NormX,
    NormY,
    NormZ,

    TexU,
    TexV,
    TexW,

    Void
}

pub struct ObjLoader{
    verts : Vec<Vec3>,
    load_verts : bool,

    normals : Vec<Vec3>,
    load_normals : bool,

    texs : Vec<Vec3>,
    load_tex : bool,

    load_data_order : Vec<BufferDataType>
}

impl ObjLoader{
    pub fn new() -> ObjLoader{
        ObjLoader {
            verts : Vec::new(),
            load_verts : false,
            normals : Vec::new(),
            load_normals : false,
            texs : Vec::new(),
            load_tex : false,
            load_data_order : Vec::new()
        }
    }

    pub fn load(&mut self, path : String, load_data_order : Vec<BufferDataType>) -> Vec<f32>{
        self.load_data_order = load_data_order;
        
        self.load_verts = false;
        self.load_normals = false;
        self.load_tex = false;

        for data_type in &self.load_data_order{
            match data_type{
                BufferDataType::PosX  | BufferDataType::PosY  | BufferDataType::PosZ  => { self.load_verts = true;  }
                BufferDataType::NormX | BufferDataType::NormY | BufferDataType::NormZ => { self.load_normals = true; }
                BufferDataType::TexU  | BufferDataType::TexV  | BufferDataType::TexW  => { self.load_tex = true;  }
                _ => { }
            }
        }

        if self.load_verts{ self.load_verts(BufReader::new(File::open(path.as_str()).unwrap())); }
        if self.load_normals{ self.load_normals(BufReader::new(File::open(path.as_str()).unwrap())); }
        if self.load_tex{ self.load_texs(BufReader::new(File::open(path.as_str()).unwrap())); }

        let mut output_data : Vec<f32> = Vec::new();

        let reader = BufReader::new(File::open(path.as_str()).unwrap());
        for line in reader.lines() {

            let line = line.unwrap();
            if line.len() < 2 { continue; }

            if line.chars().nth(0).unwrap() == 'f' && line.chars().nth(1).unwrap() == ' ' {
                let face_descr: String = line.chars().into_iter().skip(2).collect();
                output_data.append(&mut self.load_face_line(&face_descr));
            }
        }

        output_data
    }

    fn load_face_line(&self, line : &String) -> Vec<f32>{
        let mut output_data : Vec<f32> = Vec::new();
        let mut face_descr_iter = line.split_whitespace();

        let mut vert_ids : [usize; 3] = [0; 3];
        let mut normal_ids : [usize; 3] = [0; 3];
        let mut tex_ids : [usize; 3] = [0; 3];
        for i in 0..3 {
            let ids_line = face_descr_iter.next().unwrap();
            let mut ids_iter = ids_line.split("/");
            // Vert
            let vert_id = ids_iter.next().ok_or("");
            // Parse if need to load
            if self.load_verts{
                if let Ok(vert_id) = vert_id { 
                    vert_ids[i] = vert_id.parse::<usize>().unwrap(); 
                } else {
                    println!("{}", "vertex coords are missing in obj! Press enter to close window".red());
                    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
                    std::process::exit(0);
                }
            }       
            // Tex
            let tex_id = ids_iter.next().ok_or("");
            // Parse if need to load
            if self.load_tex{
                if let Ok(tex_id) = tex_id { 
                    tex_ids[i] = tex_id.parse::<usize>().unwrap(); 
                } else {
                    println!("{}", "texture coords are missing in obj! Press enter to close window".red());
                    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
                    std::process::exit(0);
                }
            }      
            // Norm
            let normal_id = ids_iter.next().ok_or("");
            // Parse if need to load
            if self.load_normals{
                if let Ok(normal_id) = normal_id { 
                    normal_ids[i] = normal_id.parse::<usize>().unwrap(); 
                } else {
                    println!("{}", "normals are missing in obj! Press enter to close window".red());
                    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
                    std::process::exit(0);
                }
            }      
        }

        let mut verts : [Vec3; 3] = [Vec3::zero(), Vec3::zero(), Vec3::zero()];
        let mut normals : [Vec3; 3] = [Vec3::zero(), Vec3::zero(), Vec3::zero()];
        let mut texs : [Vec3; 3] = [Vec3::zero(), Vec3::zero(), Vec3::zero()];

        if self.load_verts { 
            verts = [
                self.verts[vert_ids[0] - 1].clone(), 
                self.verts[vert_ids[1] - 1].clone(), 
                self.verts[vert_ids[2] - 1].clone()
            ]; 
        }                    
        if self.load_normals { 
            normals = [
                self.normals[normal_ids[0] - 1].clone(), 
                self.normals[normal_ids[1] - 1].clone(), 
                self.normals[normal_ids[2] - 1].clone()
            ]; 
        }
        if self.load_tex { 
            texs = [
                self.texs[tex_ids[0] - 1].clone(), 
                self.texs[tex_ids[1] - 1].clone(), 
                self.texs[tex_ids[2] - 1].clone()
            ]; 
        }

        for i in 0..3{
            // Push data for each vert
            for data_type in &self.load_data_order{
                match data_type{
                    BufferDataType::PosX => { output_data.push(verts[i].x); }
                    BufferDataType::PosY => { output_data.push(verts[i].y); }
                    BufferDataType::PosZ => { output_data.push(verts[i].z); }

                    BufferDataType::NormX => { output_data.push(normals[i].x); }
                    BufferDataType::NormY => { output_data.push(normals[i].y); }
                    BufferDataType::NormZ => { output_data.push(normals[i].z); }

                    BufferDataType::TexU => { output_data.push(texs[i].x); }
                    BufferDataType::TexV => { output_data.push(texs[i].y); }
                    BufferDataType::TexW => { output_data.push(texs[i].z); }

                    _ => { output_data.push(0.0); }
                }
            }
        }    

        output_data
    }

    fn load_verts(&mut self, reader: std::io::BufReader<std::fs::File>) {
        self.verts = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            if line.len() < 2 { continue; }

            if line.chars().nth(0).unwrap() == 'v' && line.chars().nth(1).unwrap() == ' ' {
                let coords: String = line.chars().into_iter().skip(2).collect();      
                self.verts.push(ObjLoader::parse_vec3(&coords));
            }
        }
    }

    fn load_normals(&mut self, reader: std::io::BufReader<std::fs::File>) {
        self.normals = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            if line.len() < 2 { continue; }

            if line.chars().nth(0).unwrap() == 'v' && line.chars().nth(1).unwrap() == 'n' {
                let coords: String = line.chars().into_iter().skip(3).collect();      
                self.normals.push(ObjLoader::parse_vec3(&coords).normalized());
            }
        }
    }

    fn load_texs(&mut self, reader: std::io::BufReader<std::fs::File>){
        self.texs = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            if line.len() < 2 { continue; }

            if line.chars().nth(0).unwrap() == 'v' && line.chars().nth(1).unwrap() == 't' {
                let texs: String = line.chars().into_iter().skip(3).collect();      
                self.texs.push(ObjLoader::parse_tex(&texs).normalized());
            }
        }
    }

    fn parse_tex(line : &String) -> Vec3{
        let mut coords_iter = line.split_whitespace();
        let u = coords_iter.next().unwrap().parse::<f32>().unwrap();
        let v = coords_iter.next().unwrap().parse::<f32>().unwrap();
        let mut tex = Vec3::new(u, v, 0.0);
        match coords_iter.next() {
            Some(w) => {
                tex.z = w.parse::<f32>().unwrap();
            }
            _ => {}
        }
        tex
    }

    fn parse_vec3(line : &String) -> Vec3{
        let mut coords_iter = line.split_whitespace();
        let x = coords_iter.next().unwrap().parse::<f32>().unwrap();
        let y = coords_iter.next().unwrap().parse::<f32>().unwrap();
        let z = coords_iter.next().unwrap().parse::<f32>().unwrap();
        let mut point = Vec3::new(x, y, z);
        match coords_iter.next() {
            Some(w) => {
                point = &point / w.parse::<f32>().unwrap();
            }
            _ => {}
        }
        point
    }
}