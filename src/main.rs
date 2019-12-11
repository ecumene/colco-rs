#![deny(clippy::all)]
use glow::*;

pub mod constants;

use glam::{Mat4, Vec3};

use std::{cell::RefCell, rc::Rc};

use regex::Regex;
use std::str::FromStr;

use std_web::console;
use std_web::{
    traits::*,
    unstable::TryInto,
    web::{
        document,
        event::{IMouseEvent, MouseDownEvent, MouseMoveEvent, MouseUpEvent},
        html_element::*,
    },
};

use webgl_stdweb::WebGL2RenderingContext;

pub fn wasm_main() {
    main();
}

#[derive(Clone)]
enum Elements {
    C,
    H,
}

#[derive(Clone)]
struct Atom {
    position: Vec3,
    element: Elements,
}

#[derive(Clone)]
struct Mol<'a> {
    atoms: Vec<Atom>,
    bonds: Vec<(&'a Atom, &'a Atom)>,
}

impl FromStr for Mol<'_> {
    type Err = regex::Error;

    fn from_str(mol: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r#"(\-?[0-9][\.][0-9]+[0-9][ \t]+)(\-?[0-9][\.][0-9]+[0-9][ \t]+)(\-?[0-9][\.][0-9]+[0-9][ \t]+)(\w)"#)?;
        let atoms = regex
            .captures_iter(mol)
            .filter_map(|cap| {
                let groups = (cap.get(1), cap.get(2), cap.get(3), cap.get(4));
                match groups {
                    (Some(x), Some(y), Some(z), Some(a)) => Some(Atom {
                        position: Vec3::new(
                            x.as_str().trim().parse().unwrap(),
                            y.as_str().trim().parse().unwrap(),
                            z.as_str().trim().parse().unwrap(),
                        ),
                        element: Elements::C,
                    }),
                    x => None,
                }
            })
            .map(|m| m)
            .collect::<Vec<_>>();
        Ok(Mol {
            atoms,
            bonds: vec![],
        })
    }
}

struct MovementInner<'a> {
    is_mouse_down: bool,
    transform: Mat4,
    mol: Mol<'a>,
}

impl<'a> Default for MovementInner<'a> {
    fn default() -> Self {
        MovementInner {
            is_mouse_down: true,
            transform: Mat4::identity(),
            mol: Mol::from_str(
                "RDKit          3D

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
  2  3  1  0
  3  4  1  0
  4  1  1  0
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
struct Movement<'a> {
    inner: Rc<RefCell<MovementInner<'a>>>,
}

#[cfg(all(target_arch = "wasm32", feature = "stdweb"))]
impl Movement<'_> {
    pub fn get_molecule(&self) -> Mol {
        self.inner.borrow().mol.clone()
    }

    pub fn get_transform(&self) -> Mat4 {
        self.inner.borrow().transform
    }

    pub fn on_mouse_move<MouseMoveEvent: IMouseEvent>(&self, location: MouseMoveEvent) {
        let mut inner = self.inner.borrow_mut();
        if (inner.is_mouse_down) {
            inner.transform =
                inner.transform * Mat4::from_rotation_x(location.movement_y() as f32 * 10.0);
        }
    }

    pub fn on_mouse_down<MouseDownEvent: IMouseEvent>(&self, location: MouseDownEvent) {
        let mut inner = self.inner.borrow_mut();
        inner.is_mouse_down = true;
        console!(log, inner.is_mouse_down);
    }

    pub fn on_mouse_up<MouseUpEvent: IMouseEvent>(&self, location: MouseUpEvent) {
        let mut inner = self.inner.borrow_mut();
        inner.is_mouse_down = false;
        console!(log, inner.is_mouse_down);
    }
}

impl Default for Movement<'_> {
    fn default() -> Self {
        Movement {
            inner: Rc::new(RefCell::new(MovementInner::default())),
        }
    }
}

fn main() {
    let movement: Movement = Movement::default();

    unsafe {
        #[cfg(all(target_arch = "wasm32", feature = "stdweb"))]
        let (_window, gl, _events_loop, render_loop, shader_version) = {
            let canvas: CanvasElement = document()
                .create_element("canvas")
                .unwrap()
                .try_into()
                .unwrap();
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
            canvas.set_width(1024);
            canvas.set_height(768);
            let webgl2_context: WebGL2RenderingContext = canvas.get_context().unwrap();
            (
                (),
                glow::Context::from_webgl2_context(webgl2_context),
                (),
                glow::RenderLoop::from_request_animation_frame(),
                "#version 300 es",
            )
        };

        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_size(
            glow::ARRAY_BUFFER,
            constants::SPHERE_MESH.len() as i32 * std::mem::size_of::<f32>() as i32,
            glow::STATIC_DRAW,
        );
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(&constants::SPHERE_MESH),
            glow::STATIC_DRAW,
        );
        gl.bind_buffer(glow::ARRAY_BUFFER, None);

        let ibo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
        gl.buffer_data_size(
            glow::ELEMENT_ARRAY_BUFFER,
            constants::SPHERE_INDICES.len() as i32 * std::mem::size_of::<u16>() as i32,
            glow::STATIC_DRAW,
        );
        gl.buffer_data_u8_slice(
            glow::ELEMENT_ARRAY_BUFFER,
            bytemuck::cast_slice(&constants::SPHERE_INDICES),
            glow::STATIC_DRAW,
        );

        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array));
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(
            0,
            3,
            glow::FLOAT,
            false,
            6 * std::mem::size_of::<f32>() as i32,
            0,
        );
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(
            1,
            3,
            glow::FLOAT,
            false,
            6 * std::mem::size_of::<f32>() as i32,
            3 * std::mem::size_of::<f32>() as i32,
        );

        let program = gl.create_program().expect("Cannot create program");

        let (vertex_shader_source, fragment_shader_source) = (
            r#"
            layout(location = 0) in vec3 vert_in;
            layout(location = 1) in vec3 norm_in;
            out vec3 norm_out;
            out vec3 eye;
            uniform mat4 transform;
            void main() {
                eye = -normalize (vert_in);
                norm_out = normalize(norm_in);
                gl_Position = transform * vec4(vert_in, 1.0);
            }"#,
            r#"precision mediump float;
            in vec3 norm_out;
            in vec3 eye;
            out vec4 color;
            void main() {
                color = vec4(vec3(dot(eye, normalize(reflect(vec3(0.0, 0.0, 1.0), norm_out)))), 1.0);
            }"#,
        );

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

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

        gl.use_program(Some(program));
        gl.clear_color(0.0, 0.0, 0.0, 0.0);

        let movement_for_render_closure = movement.clone();

        render_loop.run(move |running: &mut bool| {
            gl.clear(glow::COLOR_BUFFER_BIT);
            for atom in movement_for_render_closure.get_molecule().atoms {
                let transform_location = gl.get_uniform_location(program, "transform");
                gl.uniform_matrix_4_f32_slice(
                    transform_location,
                    false,
                    (
                        Mat4::from_translation(atom.position * 0.25)
                        * Mat4::from_scale(Vec3::new(0.1, 0.1, 0.1))
                        // * Mat4::orthographic_rh_gl(-10.0, 10.0, -10.0, 10.0, -10.0, 10.0)
                        // * Mat4::from_rotation_x(0)
                        )
                    .as_ref(),
                );
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
                gl.draw_elements(glow::TRIANGLES, constants::SPHERE_INDICES.len() as i32, glow::UNSIGNED_SHORT, 0);
            }
            if !*running {
                gl.delete_program(program);
                gl.delete_vertex_array(vertex_array);
            }
        });
    }
}
