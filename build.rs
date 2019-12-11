use build_const::ConstWriter;
use std::fmt;
use std::path::Path;
use itertools::{interleave, Itertools};

fn main() {
    let mut consts = ConstWriter::from_path(&Path::new("src/constants.rs"))
        .unwrap()
        .finish_dependencies();

    let sphere = tobj::load_obj(&Path::new("sphere.obj"));
    assert!(sphere.is_ok());
    let (models, materials) = sphere.unwrap();

    for (i, m) in models.iter().enumerate() {
        let data = interleave(&(&m.mesh.positions).into_iter().chunks(3), &(&m.mesh.normals).into_iter().chunks(3)).map(|v| {v.into_iter().cloned().collect::<Vec<f32>>()}).flatten().collect::<Vec<f32>>();
        consts.add_array("SPHERE_MESH", "f32", &data);
        consts.add_array("SPHERE_INDICES", "u16", &m.mesh.indices);
    }
}
