use build_const::ConstWriter;
use itertools::{interleave, Itertools};
use std::fmt;
use std::path::Path;

fn main() {
    let mut consts = ConstWriter::from_path(&Path::new("src/constants.rs"))
        .unwrap()
        .finish_dependencies();

    let sphere = tobj::load_obj(&Path::new("sphere.obj"));
    assert!(sphere.is_ok());
    let cylinder = tobj::load_obj(&Path::new("cylinder.obj"));
    assert!(cylinder.is_ok());

    let mut mesh = vec![];
    let mut indices = vec![];

    // TODO: These could be one iterator via chain()
    let (models, materials) = sphere.unwrap();
    for (i, m) in models.iter().enumerate() {
        let data = interleave(
            &(&m.mesh.positions).into_iter().chunks(3),
            &(&m.mesh.normals).into_iter().chunks(3),
        )
        .flatten()
        .collect::<Vec<_>>();
        mesh.append(&mut data.clone());
        indices.append(&mut m.mesh.indices.clone());
        consts.add_value("SPHERE_SIZE", "usize", indices.len());
    }

    let (models, materials) = cylinder.unwrap();
    for (i, m) in models.iter().enumerate() {
        let data = interleave(
            &(&m.mesh.positions).into_iter().chunks(3),
            &(&m.mesh.normals).into_iter().chunks(3),
        )
        .flatten()
        .collect::<Vec<_>>();
        let current_index = indices.iter().max().unwrap() + 1;
        mesh.append(&mut data.clone());
        indices.append(
            &mut m
                .mesh
                .indices
                .clone()
                .into_iter()
                .map(|m| m + current_index)
                .collect::<Vec<_>>(),
        )
    }

    consts.add_array("MESH", "f32", &mesh);
    consts.add_array("INDICES", "u32", &indices);
}
