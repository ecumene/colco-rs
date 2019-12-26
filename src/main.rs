// TODO: Reduce bundle size from 890kb to ~300kb

#![deny(clippy::all)]
use glow::*;
use glam::{Mat4, Quat, Vec3};
use std::{cell::RefCell, rc::Rc};
use webgl_stdweb::WebGL2RenderingContext;
use std::str::FromStr;
use std_web::{
    traits::*,
    unstable::TryInto,
    console,
    web::{
        document,
        event::{IMouseEvent, MouseDownEvent, MouseMoveEvent, MouseUpEvent},
        html_element::*,
    },
};

pub mod constants;
pub mod assets;
pub mod mol;

use constants::{SPHERE_SIZE, INDICES};
use mol::Mol;
use assets::init_buffers_from_constants;

pub fn wasm_main() {
    main();
}

struct ColcoInner {
    is_mouse_down: bool,
    rotation: Quat,
    mol: Mol,
}

impl Default for ColcoInner {
    fn default() -> Self {
        ColcoInner {
            is_mouse_down: false,
            rotation: Quat::from_xyzw(0.0, 1.0, 0.0, 0.0),
            // TODO: Mol from JS
            mol: Mol::from_str(
                "
                RDKit          3D

                12 12  0  0  0  0  0  0  0  0999 V2000
                   0.4280    0.9580   -0.0542 C   0  0  0  0  0  0  0  0  0  0  0  0
                   0.9494   -0.4476    0.0775 C   0  0  0  0  0  0  0  0  0  0  0  0
                  -0.4453   -0.9477   -0.1840 C   0  0  0  0  0  0  0  0  0  0  0  0
                  -0.9600    0.4286    0.2195 C   0  0  0  0  0  0  0  0  0  0  0  0
                   0.8336    1.6583    0.6806 H   0  0  0  0  0  0  0  0  0  0  0  0
                   0.5192    1.3314   -1.1071 H   0  0  0  0  0  0  0  0  0  0  0  0
                   1.6987   -0.6701   -0.7153 H   0  0  0  0  0  0  0  0  0  0  0  0
                   1.3564   -0.7346    1.0500 H   0  0  0  0  0  0  0  0  0  0  0  0
                  -0.7635   -1.7281    0.5135 H   0  0  0  0  0  0  0  0  0  0  0  0
                  -0.5819   -1.1411   -1.2610 H   0  0  0  0  0  0  0  0  0  0  0  0
                  -1.7380    0.7914   -0.4921 H   0  0  0  0  0  0  0  0  0  0  0  0
                  -1.2964    0.5016    1.2727 H   0  0  0  0  0  0  0  0  0  0  0  0
                 1  2  1  0
                 2  3  2  0
                 3  4  3  0
                 4  1  4  0
                 1  5  1  0
                 1  6  1  0
                 2  7  1  0
                 2  8  1  0
                 3  9  1  0
                 3 10  1  0
                 4 11  1  0
                 4 12  1  0
               M  END
                ",
            )
            .unwrap(),
        }
    }
}

#[derive(Clone)]
struct Colco {
    inner: Rc<RefCell<ColcoInner>>,
}

impl Colco {
    pub fn view_vector(&self) -> Vec3 {
        // TODO: Make vec constant
        let inner = self.inner.borrow();
        inner.rotation * Vec3::new(0.0, 0.0, 1.0)
    }

    pub fn render_mol<B: glow::HasContext>(
        &self,
        program: <B as glow::HasContext>::Program,
        gl: &B,
    ) {
        unsafe {
            let inner = self.inner.borrow();
            for atom in &inner.mol.atoms {
                let transform_location = gl.get_uniform_location(program, "transform");
                let color_location = gl.get_uniform_location(program, "u_color");
                &gl.uniform_3_f32(
                    color_location,
                    atom.element.color.x(),
                    atom.element.color.y(),
                    atom.element.color.z(),
                );
                let view_location = gl.get_uniform_location(program, "u_view");
                let view_vector = self.view_vector();
                &gl.uniform_3_f32(
                    view_location,
                    view_vector.x(),
                    view_vector.y(),
                    view_vector.z(),
                );
                &gl.uniform_matrix_4_f32_slice(
                    transform_location,
                    false,
                    (inner.mol.bounding_projection
                        * Mat4::from_quat(inner.rotation).transpose()
                        * Mat4::from_translation(atom.position * 4.5)
                        * Mat4::from_scale(
                            Vec3::new(atom.element.scale, atom.element.scale, atom.element.scale)
                                * 2.0,
                        ))
                    .as_ref(),
                );
                &gl.draw_elements(
                    glow::TRIANGLES,
                    SPHERE_SIZE as i32,
                    glow::UNSIGNED_INT,
                    0,
                );
            }
            for bond in &inner.mol.bonds {
                // TODO: Move uniform locations into shader-lifetime-constrained constants
                let transform_location = gl.get_uniform_location(program, "transform");
                let color_location = gl.get_uniform_location(program, "u_color");
                &gl.uniform_3_f32(color_location, 0.25, 0.25, 0.25);
                for bond_num in 0..bond.bond_type {
                    &gl.uniform_matrix_4_f32_slice(
                        transform_location.clone(),
                        false,
                        (inner.mol.bounding_projection
                            * Mat4::from_quat(inner.rotation).transpose()
                            * Mat4::from_translation(bond.position  * 4.5)
                            * Mat4::from_quat(bond.rotation)
                            * Mat4::from_translation(Vec3::new(0.0, 0.0, 0.6 * bond_num as f32 - (0.25 * bond.bond_type as f32)))
                            * Mat4::from_scale(Vec3::new(0.5 / bond.bond_type as f32, bond.length * 2.25, 0.5 / bond.bond_type as f32)))
                        .as_ref(),
                    );
                    &gl.draw_elements(
                        glow::TRIANGLES,
                        (INDICES.len() - SPHERE_SIZE) as i32,
                        glow::UNSIGNED_INT,
                        (SPHERE_SIZE * std::mem::size_of::<u32>()) as i32,
                    );
                }
            }
        }
    }

    // TODO: Decouple, for desktop version
    pub fn on_mouse_move<MouseMoveEvent: IMouseEvent>(&self, location: MouseMoveEvent) {
        let mut inner = self.inner.borrow_mut();
        if inner.is_mouse_down {
            inner.rotation = inner.rotation
                * Quat::from_rotation_x(-location.movement_y() as f32 * 0.005)
                * Quat::from_rotation_y(-location.movement_x() as f32 * 0.005);
        }
    }

    pub fn on_mouse_down<MouseDownEvent: IMouseEvent>(&self, _location: MouseDownEvent) {
        let mut inner = self.inner.borrow_mut();
        inner.is_mouse_down = true;
    }

    pub fn on_mouse_up<MouseUpEvent: IMouseEvent>(&self, _location: MouseUpEvent) {
        let mut inner = self.inner.borrow_mut();
        inner.is_mouse_down = false;
    }
}

impl Default for Colco {
    fn default() -> Self {
        Colco {
            inner: Rc::new(RefCell::new(ColcoInner::default())),
        }
    }
}

fn main() {
    let movement: Colco = Colco::default();

    unsafe {
        let (_window, gl, _events_loop, render_loop, shader_version) = {
            let canvas: CanvasElement = document()
                .create_element("canvas")
                .unwrap()
                .try_into()
                .unwrap();
            // TODO: Decouple for desktop version
            let movement_for_mouse_move_event_closure = movement.clone();
            canvas.add_event_listener(move |event: MouseMoveEvent| {
                movement_for_mouse_move_event_closure.on_mouse_move(event);
            });
            let movement_for_mouse_down_event_closure = movement.clone();
            canvas.add_event_listener(move |event: MouseDownEvent| {
                movement_for_mouse_down_event_closure.on_mouse_down(event);
            });
            let movement_for_mouse_up_event_closure = movement.clone();
            canvas.add_event_listener(move |event: MouseUpEvent| {
                movement_for_mouse_up_event_closure.on_mouse_up(event);
            });
            document().body().unwrap().append_child(&canvas);
            // TODO: JS defined size
            canvas.set_width(768);
            canvas.set_height(768);
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
            (glow::VERTEX_SHADER, r#"
            layout(location = 0) in vec3 vert_in;
            layout(location = 1) in vec3 norm_in;
            out vec3 norm_out;
            out vec3 eye;
            uniform mat4 transform;
            void main() {
                eye = -normalize (vert_in);
                norm_out = normalize(norm_in);
                gl_Position = transform * vec4(vert_in, 1.0);
            }"#),
            (glow::FRAGMENT_SHADER, r#"precision mediump float;
            uniform vec3 u_color;
            uniform vec3 u_view;
            in vec3 norm_out;
            in vec3 eye;
            out vec4 color;
            void main() {
                color = vec4(u_color * vec3(dot(eye, normalize(reflect(u_view, norm_out)))), 1.0);
            }"#),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        // TODO: Good error handling
        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!(gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!(gl.get_program_info_log(program));
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

        let movement_for_render_closure = movement.clone();
        let vertex_array = init_buffers_from_constants(&gl);

        render_loop.run(move |running: &mut bool| {
            gl.clear(glow::COLOR_BUFFER_BIT);
            movement_for_render_closure.render_mol(program, &gl);
            if !*running {
                gl.delete_program(program);
                gl.delete_vertex_array(vertex_array);
            }
        });
    }
}
