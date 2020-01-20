// TODO: Reduce bundle size from 890kb to ~300kb

#![deny(clippy::all)]
use glam::{Mat4, Quat, Vec3};
use glow::HasContext as Context;
use glow::*;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::str::FromStr;
use std::sync::Mutex;
use stdweb::{
    console, js_deserializable, js_export,
    traits::*,
    unstable::TryInto,
    web::{
        document,
        event::{IMouseEvent, MouseDownEvent, MouseMoveEvent, MouseUpEvent},
        html_element::*,
    },
};
use webgl_stdweb::WebGL2RenderingContext;

pub mod assets;
pub mod constants;
pub mod mol;

use assets::init_buffers_from_constants;
use constants::{MESHES_SIZE, SPHERE_SIZE};
use mol::Mol;

struct Colco {
    is_mouse_down: bool,
    rotation: Quat,
    mol: Mol,
}

impl Colco {
    fn new(mol: Mol) -> Self {
        Colco {
            is_mouse_down: false,
            rotation: Quat::from_xyzw(0.0, 1.0, 0.0, 0.0),
            mol,
        }
    }
}

impl Colco {
    pub unsafe fn render_mol<B: Context>(
        &self,
        settings: &RenderSettings,
        transform_uniform: &Option<<B as Context>::UniformLocation>,
        light_uniform: &Option<<B as Context>::UniformLocation>,
        color_uniform: &Option<<B as Context>::UniformLocation>,
        view_uniform: &Option<<B as Context>::UniformLocation>,
        gl: &B,
    ) {
        let view_vector = self.rotation * Vec3::unit_z();
        let view_location = Some(view_uniform.as_ref().unwrap().clone());
        &gl.uniform_3_f32(
            view_location,
            view_vector.x(),
            view_vector.y(),
            view_vector.z(),
        );
        let light_location_true = Some(light_uniform.as_ref().unwrap().clone());
        &gl.uniform_1_i32(light_location_true, glow::TRUE as i32);
        for atom in &self.mol.atoms {
            let transform_location = Some(transform_uniform.as_ref().unwrap().clone());
            let color_location = Some(color_uniform.as_ref().unwrap().clone());
            &gl.uniform_3_f32(
                color_location,
                atom.element.color.x(),
                atom.element.color.y(),
                atom.element.color.z(),
            );
            &gl.uniform_matrix_4_f32_slice(
                transform_location,
                false,
                (self.mol.bounding_projection
                    * Mat4::from_quat(self.rotation).transpose()
                    * Mat4::from_translation(atom.position * 4.5)
                    * Mat4::from_scale(
                        Vec3::new(atom.element.scale, atom.element.scale, atom.element.scale)
                            * settings.atom_size,
                    ))
                .as_ref(),
            );
            &gl.draw_elements(glow::TRIANGLES, SPHERE_SIZE as i32, glow::UNSIGNED_INT, 0);
        }
        let light_location_false = Some(light_uniform.as_ref().unwrap().clone());
        &gl.uniform_1_i32(light_location_false, glow::FALSE as i32);
        for bond in &self.mol.bonds {
            let transform_location = Some(transform_uniform.as_ref().unwrap().clone());
            for bond_num in 0..bond.bond_type {
                let color_location = Some(color_uniform.as_ref().unwrap().clone());
                &gl.uniform_3_f32(
                    color_location,
                    bond.from_color.x(),
                    bond.from_color.y(),
                    bond.from_color.z(),
                );
                &gl.uniform_matrix_4_f32_slice(
                    transform_location.clone(),
                    false,
                    (self.mol.bounding_projection
                        * Mat4::from_quat(self.rotation).transpose()
                        * Mat4::from_translation(bond.position * 4.5)
                        * Mat4::from_quat(bond.rotation)
                        * Mat4::from_translation(Vec3::new(
                            bond_num as f32 - (0.5 * (bond.bond_type - 1) as f32),
                            0.0,
                            0.0,
                        ))
                        * Mat4::from_scale(Vec3::new(
                            settings.bond_size / bond.bond_type as f32,
                            bond.length * 1.15,
                            settings.bond_size / bond.bond_type as f32,
                        )))
                    .as_ref(),
                );
                &gl.draw_elements(
                    glow::TRIANGLES,
                    (MESHES_SIZE - SPHERE_SIZE) as i32,
                    glow::UNSIGNED_INT,
                    (SPHERE_SIZE * std::mem::size_of::<u32>()) as i32,
                );
                let color_location = Some(color_uniform.as_ref().unwrap().clone());
                &gl.uniform_3_f32(
                    color_location,
                    bond.to_color.x(),
                    bond.to_color.y(),
                    bond.to_color.z(),
                );
                &gl.uniform_matrix_4_f32_slice(
                    transform_location.clone(),
                    false,
                    (self.mol.bounding_projection
                        * Mat4::from_quat(self.rotation).transpose()
                        * Mat4::from_translation(bond.position * 4.5)
                        * Mat4::from_quat(bond.rotation)
                        * Mat4::from_translation(Vec3::new(
                            bond_num as f32 - (0.5 * (bond.bond_type - 1) as f32),
                            bond.length * 2.25,
                            0.0,
                        ))
                        * Mat4::from_scale(Vec3::new(
                            settings.bond_size / bond.bond_type as f32,
                            bond.length * 1.15,
                            settings.bond_size / bond.bond_type as f32,
                        )))
                    .as_ref(),
                );
                &gl.draw_elements(
                    glow::TRIANGLES,
                    (MESHES_SIZE - SPHERE_SIZE) as i32,
                    glow::UNSIGNED_INT,
                    (SPHERE_SIZE * std::mem::size_of::<u32>()) as i32,
                );
            }
        }
    }

    // TODO: Decouple, for desktop version
    pub fn on_mouse_move<MouseMoveEvent: IMouseEvent>(&mut self, location: MouseMoveEvent) {
        if self.is_mouse_down {
            self.rotation = self.rotation
                * Quat::from_rotation_x(-location.movement_y() as f32 * 0.025)
                * Quat::from_rotation_y(-location.movement_x() as f32 * 0.025);
        }
    }

    pub fn on_mouse_down<MouseDownEvent: IMouseEvent>(&mut self, _location: MouseDownEvent) {
        self.is_mouse_down = true;
    }

    pub fn on_mouse_up<MouseUpEvent: IMouseEvent>(&mut self, _location: MouseUpEvent) {
        self.is_mouse_down = false;
    }
}

#[derive(Deserialize, Debug, Clone)]
struct RenderSettings {
    atom_size: f32,
    bond_size: f32,
}

lazy_static! {
    static ref colco: Mutex<Option<Colco>> = Mutex::new(None);
    static ref render_settings: Mutex<Option<RenderSettings>> = Mutex::new(Some(RenderSettings {
        atom_size: 2.0,
        bond_size: 0.5,
    }));
}

js_deserializable!(RenderSettings);

// TODO: Get render_settings via js for spreading new settings into old ones

#[js_export]
fn setRenderSettings(render: RenderSettings) {
    let mut settings = render_settings.lock().unwrap();
    *settings = Some(render);
}

#[js_export]
fn setMolecule(molecule_data: &str) {
    let mut state = colco.lock().unwrap();
    *state = Some(Colco::new(Mol::from_str(molecule_data).unwrap()));
}

#[js_export]
fn render(element_id: &str) {
    // Todo: wrap unsafe around native fns only
    unsafe {
        let (_window, gl, _events_loop, render_loop, shader_version) = {
            let canvas: CanvasElement = document()
                .get_element_by_id(element_id)
                .unwrap()
                .try_into()
                .unwrap();
            // TODO: Decouple for desktop version
            canvas.add_event_listener(move |event: MouseMoveEvent| {
                colco.lock().unwrap().as_mut().unwrap().on_mouse_move(event);
            });
            canvas.add_event_listener(move |event: MouseDownEvent| {
                colco.lock().unwrap().as_mut().unwrap().on_mouse_down(event);
            });
            canvas.add_event_listener(move |event: MouseUpEvent| {
                colco.lock().unwrap().as_mut().unwrap().on_mouse_up(event);
            });
            document().body().unwrap().append_child(&canvas);
            // TODO: Desktop context
            let webgl2_context: WebGL2RenderingContext = canvas.get_context().unwrap();
            (
                (),
                glow::Context::from_webgl2_context(webgl2_context),
                (),
                glow::RenderLoop::from_request_animation_frame(),
                "#version 300 es",
            )
        };

        let program = gl.create_program().expect("Cannot create program");

        let shader_sources = [
            (
                glow::VERTEX_SHADER,
                r#"layout(location = 0) in vec3 vert_in;
                layout(location = 1) in vec3 norm_in;
                out vec3 norm_out;
                out vec3 vert_out;
                uniform mat4 transform;
                void main() {
                    vert_out = -normalize (vert_in);
                    norm_out = norm_in;
                    gl_Position = transform * vec4(vert_in, 1.0);
                }"#,
            ),
            (
                glow::FRAGMENT_SHADER,
                r#"precision mediump float;
                uniform vec3 u_color;
                uniform bool u_light;
                uniform vec3 u_view;
                in vec3 norm_out;
                in vec3 vert_out;
                out vec4 color;
                void main() {
                    if (u_light) {
                        color = vec4(u_color * 0.1 * vec3(dot(vert_out, normalize(reflect(u_view, norm_out)))) + u_color * 0.9, 1.0);
                    } else {
                        color = vec4(u_color, 1.0);
                    }
                }"#,
            ),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                console!(error, gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            console!(error, gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        // TODO: Compile flags for color, cullface, enables...
        gl.use_program(Some(program));
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.enable(glow::CULL_FACE);
        gl.cull_face(glow::BACK);
        gl.enable(glow::DEPTH_TEST);

        let vertex_array = init_buffers_from_constants(&gl);
        let transform_location = gl.get_uniform_location(program, "transform");
        let light_uniform = gl.get_uniform_location(program, "u_light");
        let color_location = gl.get_uniform_location(program, "u_color");
        let view_location = gl.get_uniform_location(program, "u_view");

        render_loop.run(move |running: &mut bool| {
            gl.clear(glow::COLOR_BUFFER_BIT);
            colco.lock().unwrap().as_ref().unwrap().render_mol(
                &render_settings.lock().unwrap().as_ref().unwrap(),
                &transform_location,
                &light_uniform,
                &color_location,
                &view_location,
                &gl,
            );
            if !*running {
                gl.delete_program(program);
                gl.delete_vertex_array(vertex_array);
            }
        });
    }
}

fn main() {
    console!(
        log,
        "Colco loaded. Consider contributing - https://github.com/ecumene/colco-rs"
    )
}
