use std::vec;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn load_from_file (fileName : &str) -> ( Vec<f32>, Vec<u32>) {

    let f = File::open(fileName).expect("file not found");
    let file = BufReader::new(&f);
    let lines = file.lines().map(|s| s.unwrap().to_string());

    let mut vertex : Vec<f32> = Vec::new();
    let mut index : Vec<u32> = Vec::new();
    
    for l in lines {
        let words : Vec<&str> = l.split(" ").collect();
        let type_of_line = words[0];
    
        if type_of_line == "#"{
            continue;
        }

        if type_of_line == "v" {
            vertex.push( words[1].parse().unwrap() );
            vertex.push( words[2].parse().unwrap() );
            vertex.push( words[3].parse().unwrap() );
        }
        if type_of_line == "f" {
            index.push( words[1].parse().unwrap() );
            index.push( words[2].parse().unwrap() );
            index.push( words[3].parse().unwrap() );
        }
    }

    (vertex, index)
}
