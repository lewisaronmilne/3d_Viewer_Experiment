use std;

pub fn read_bytes(file_loc: &str) -> std::vec::Vec<u8>
{
	use std::fs::File;
    use std::io::Read;

    let file_tot_loc = [::ROOT_DIR, "/", file_loc].concat();

    let mut data = Vec::new();
    let mut file_reader = File::open(&file_tot_loc).expect("eRRoR: Unable to read file.");
    file_reader.read_to_end(&mut data).expect("eRRoR: Unable to read data.");

    data
}

pub fn read_text(file_loc: &str) -> String
{
    let byte_data = read_bytes(file_loc);
    let string_data = match String::from_utf8(byte_data)
    {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    string_data
}

pub fn load_mesh_data(meshes_folder: &str, mesh_file: &str) -> (String, Vec<::engine::Vertex>, Vec<u32>)
{
    use serde_json;
    let data = read_text(&[meshes_folder, "/", mesh_file].concat());

    let data_json: serde_json::Value = serde_json::from_str(&["{", &data, "}"].concat()).unwrap();

    let texture_filename = data_json["texture"].as_str().unwrap();
    let texture_loc = [meshes_folder, "/textures/", &texture_filename].concat();

    let vertices_len = data_json["vertices"].as_array().unwrap().len();
    let indices_len = data_json["indices"].as_array().unwrap().len();

    let mut vertices = Vec::with_capacity(vertices_len);
    let mut indices = Vec::with_capacity(indices_len);

    for v in 0..vertices_len
    {
        let vertex_data_raw = 
        (
            data_json["vertices"][v][0].as_f64().unwrap() as f32,
            data_json["vertices"][v][1].as_f64().unwrap() as f32,
            data_json["vertices"][v][2].as_f64().unwrap() as f32,
            data_json["vertices"][v][3].as_f64().unwrap() as f32,
            data_json["vertices"][v][4].as_f64().unwrap() as f32,
        );
        vertices.push(::engine::Vertex
        {
            pos: [vertex_data_raw.0, vertex_data_raw.1, vertex_data_raw.2],
            tex_coord: [vertex_data_raw.3, vertex_data_raw.4],  
        });
    }

    for i in 0..indices_len
    {
        indices.push(data_json["indices"][i].as_i64().unwrap() as u32);
    }

    (texture_loc, vertices, indices)
}