
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use mesh;

pub fn load_from_file (file_name : &str) -> mesh::Mesh {

    let f = File::open(file_name).expect("file not found");
    let file = BufReader::new(&f);
    let lines = file.lines().map(|s| s.unwrap().to_string());

    let mut vertex      : Vec<f32> = Vec::new();
    let mut v_index     : Vec<u32> = Vec::new();
    let mut normals     : Vec<f32> = Vec::new();
    let mut n_index     : Vec<u32> = Vec::new();
    let mut texture     : Vec<f32> = Vec::new();
    let mut t_index     : Vec<u32> = Vec::new();
    let mut vertex_type : mesh::VertexType = mesh::VertexType::POSITION;

    for l in lines {
        let words : Vec<&str> = l.split_whitespace().collect();

        if words.len() == 0{
            continue;
        }

        let type_of_line = words[0];

        if type_of_line == "#"{
            continue;
        }

        if type_of_line == "v" {
            vertex.push( words[1].parse().unwrap() );
            vertex.push( words[2].parse().unwrap() );
            vertex.push( words[3].parse().unwrap() );
        }

        if type_of_line == "vn" {
            normals.push( words[1].parse().unwrap() );
            normals.push( words[2].parse().unwrap() );
            normals.push( words[3].parse().unwrap() );
        }

        if type_of_line == "vt" {
            texture.push( words[1].parse().unwrap() );
            texture.push( words[2].parse().unwrap() );
        }

        if type_of_line == "f" {

            let i_words_1 : Vec<&str> = words[1].split("/").collect();
            let i_words_2 : Vec<&str> = words[2].split("/").collect();
            let i_words_3 : Vec<&str> = words[3].split("/").collect();

            v_index.push( i_words_1[0].parse::<u32>().unwrap() - 1);
            v_index.push( i_words_2[0].parse::<u32>().unwrap() - 1);
            v_index.push( i_words_3[0].parse::<u32>().unwrap() - 1);

            if i_words_1.len() > 1{
                vertex_type |= mesh::VertexType::TEXTURE | mesh::VertexType::NORMALE;

                t_index.push( i_words_1[1].parse::<u32>().unwrap() - 1);
                t_index.push( i_words_2[1].parse::<u32>().unwrap() - 1);
                t_index.push( i_words_3[1].parse::<u32>().unwrap() - 1);

                n_index.push( i_words_1[2].parse::<u32>().unwrap() - 1);
                n_index.push( i_words_2[2].parse::<u32>().unwrap() - 1);
                n_index.push( i_words_3[2].parse::<u32>().unwrap() - 1);
            }

        }
    }

    let mut r_vertex : Vec<f32> = Vec::new();
    let mut r_index  : Vec<u32> = Vec::new();
    if vertex_type != mesh::VertexType::POSITION {
        for i in 0..v_index.len(){
            let v_n = v_index[i];
            let t_n = t_index[i];
            let n_n = n_index[i];

            r_vertex.push(vertex[(v_n * 3 + 0) as usize]);
            r_vertex.push(vertex[(v_n * 3 + 1) as usize]);
            r_vertex.push(vertex[(v_n * 3 + 2) as usize]);

            r_vertex.push(texture[(t_n * 2 + 0) as usize]);
            r_vertex.push(texture[(t_n * 2 + 1) as usize]);
            
            r_vertex.push(normals[(n_n * 3 + 0) as usize]);
            r_vertex.push(normals[(n_n * 3 + 1) as usize]);            
            r_vertex.push(normals[(n_n * 3 + 2) as usize]);

            r_index.push(i as u32)
        }
        //Todo remove similar vertex

        return mesh::Mesh::new (r_vertex, r_index, vertex_type )
    }

    mesh::Mesh::new (vertex, v_index, vertex_type )
}
