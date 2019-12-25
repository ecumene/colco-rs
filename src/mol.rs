use regex::Regex;
use std::str::FromStr;
use glam::{Mat4, Quat, Vec3};

pub struct Element {
  pub color: Vec3,
  pub scale: f32,
}

pub struct Bond {
  pub position: Vec3,
  pub rotation: Quat,
  pub bond_type: u8,
  pub length: f32,
}

pub struct Atom {
  pub position: Vec3,
  pub element: Element,
}

pub struct Mol {
    pub bounding_projection: Mat4,
    pub bounding_size: f32,
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
}

impl FromStr for Mol {
    type Err = regex::Error;

    fn from_str(mol: &str) -> Result<Self, Self::Err> {
        let mut bounding_size = 0.0;
        let atoms = Regex::new(r#"(\-?[0-9][\.][0-9]+[0-9][ \t]+)(\-?[0-9][\.][0-9]+[0-9][ \t]+)(\-?[0-9][\.][0-9]+[0-9][ \t]+)(\w)"#)?
            .captures_iter(mol)
            .filter_map(|cap| {
                let groups = (cap.get(1), cap.get(2), cap.get(3), cap.get(4));
                match groups {
                    (Some(x), Some(y), Some(z), Some(a)) => {
                        let position = Vec3::new(
                            x.as_str().trim().parse().unwrap(),
                            y.as_str().trim().parse().unwrap(),
                            z.as_str().trim().parse().unwrap(),
                        );
                        let max_element = position.max_element();
                        if max_element > bounding_size {
                            bounding_size = max_element;
                        }
                        Some(Atom {
                            position,
                            element: match a.as_str().trim() {
                                "C" => Element {
                                    color: Vec3::new(0.15, 0.15, 0.15),
                                    scale: 1.0,
                                },
                                "H" => Element {
                                    color: Vec3::new(1.0, 1.0, 1.0),
                                    scale: 0.8,
                                },
                                _ => Element {
                                    color: Vec3::new(1.0, 1.0, 1.0),
                                    scale: 1.0,
                                },
                            },
                        }
                    )},
                    _x => None,
                }
            })
            .map(|m| m)
            .collect::<Vec<_>>();

        let bonds =
            // std regex doesn't support lookbehinds, so we match for the line before
            // ours to have a number at the end (i.e [0-9]\s)
            Regex::new(r#"(?mi)^\s*((?:0|[1-9])[0-9]*)\s+((?:0|[1-9])[0-9]*)\s+((?:0|[1-9])[0-9]*)\s+((?:0|[1-9])[0-9]*)\s*$"#)?
                .captures_iter(mol)
                .filter_map(|cap| {
                    let groups = (cap.get(1), cap.get(2), cap.get(3), cap.get(4));
                    match groups {
                        (Some(first_atom), Some(second_atom), Some(bond_type), Some(_useless_zeroes_mol_gives_me)) => {
                            let first_atom_index: usize =
                                first_atom.as_str().trim().parse().unwrap();
                            let second_atom_index: usize =
                                second_atom.as_str().trim().parse().unwrap();
                            let position = atoms[first_atom_index - 1].position;
                            let dest = atoms[second_atom_index - 1].position.clone() - position.clone();
                            let forward = (dest).normalize();
                            let dot = forward.dot(Vec3::unit_y());
                            let angle = dot.acos();
                            let axis = Vec3::unit_y().cross(forward).normalize();
                            Some(Bond {
                                position,
                                rotation: Quat::from_axis_angle(axis, angle),
                                bond_type: bond_type.as_str().trim().parse().unwrap(),
                                length: dest.length()
                            })
                        }
                        _x => None,
                    }
                })
                .map(|m| m)
                .collect::<Vec<_>>();
        let bounding_projection = Mat4::orthographic_rh_gl(
            -bounding_size * 4.5 * 2.0,
            bounding_size * 4.5 * 2.0,
            -bounding_size * 4.5 * 2.0,
            bounding_size * 4.5 * 2.0,
            -bounding_size * 4.5 * 2.0,
            bounding_size * 4.5 * 2.0,
        );
        Ok(Mol {
            bounding_projection,
            bounding_size,
            atoms,
            bonds,
        })
    }
}
