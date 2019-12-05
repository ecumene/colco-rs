use glow::*;

#[cfg(all(target_arch = "wasm32", feature = "web-sys"))]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "stdweb"))]
use std_web::{
    traits::*,
    unstable::TryInto,
    web::{document, html_element::*},
};
#[cfg(all(target_arch = "wasm32", feature = "stdweb"))]
use webgl_stdweb::WebGL2RenderingContext;

#[cfg_attr(all(target_arch = "wasm32", feature = "web-sys"), wasm_bindgen(start))]
pub fn wasm_main() {
    main();
}

fn main() {
    let VERTICES: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];

    unsafe {
        // Create a context from a WebGL2 context on wasm32 targets
        #[cfg(all(target_arch = "wasm32", feature = "web-sys"))]
        let (_window, gl, _events_loop, render_loop, shader_version) = {
            use wasm_bindgen::JsCast;
            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();
            let webgl2_context = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .unwrap();
            (
                (),
                glow::Context::from_webgl2_context(webgl2_context),
                (),
                glow::RenderLoop::from_request_animation_frame(),
                "#version 300 es",
            )
        };

        #[cfg(all(target_arch = "wasm32", feature = "stdweb"))]
        let (_window, gl, _events_loop, render_loop, shader_version) = {
            let canvas: CanvasElement = document()
                .create_element("canvas")
                .unwrap()
                .try_into()
                .unwrap();
            document()
                .body()
                .unwrap()
                .append_child(&canvas);
            canvas.set_width(640);
            canvas.set_height(480);
            let webgl2_context: WebGL2RenderingContext = canvas
                .get_context()
                .unwrap();
            (
                (),
                glow::Context::from_webgl2_context(webgl2_context),
                (),
                glow::RenderLoop::from_request_animation_frame(),
                "#version 300 es",
            )
        };

        // Create a context from a glutin window on non-wasm32 targets
        #[cfg(feature = "window-glutin")]
        let (gl, event_loop, windowed_context, shader_version) = {
            let el = glutin::event_loop::EventLoop::new();
            let wb = glutin::window::WindowBuilder::new()
                .with_title("Hello triangle!")
                .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
            let windowed_context = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(wb, &el)
                .unwrap();
            let windowed_context = windowed_context.make_current().unwrap();
            let context = glow::Context::from_loader_function(|s| {
                windowed_context.get_proc_address(s) as *const _
            });
            (context, el, windowed_context, "#version 410")
        };

        // Create a context from a sdl2 window
        #[cfg(feature = "window-sdl2")]
        let (gl, mut events_loop, render_loop, shader_version, _gl_context) = {
            let sdl = sdl2::init().unwrap();
            let video = sdl.video().unwrap();
            let gl_attr = video.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 0);

            let window = video
                .window("Hello triangle!", 1024, 769)
                .opengl()
                .resizable()
                .build()
                .unwrap();
            let gl_context = window.gl_create_context().unwrap();
            let context =
                glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
            let render_loop = glow::RenderLoop::<sdl2::video::Window>::from_sdl_window(window);
            let event_loop = sdl.event_pump().unwrap();
            (context, event_loop, render_loop, "#version 410", gl_context)
        };

        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_size(glow::ARRAY_BUFFER, VERTICES.len() as i32, glow::STATIC_DRAW);
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&VERTICES), glow::STATIC_DRAW);
        gl.bind_buffer(glow::ARRAY_BUFFER, None);

        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 3 * std::mem::size_of::<f32>() as i32, 0);

        let program = gl.create_program().expect("Cannot create program");

        let (vertex_shader_source, fragment_shader_source) = (
            r#"
            in vec3 vert_in;
            void main() {
                gl_Position = vec4(vert_in, 1.0);
            }"#,
            r#"precision mediump float;
            out vec4 color;
            void main() {
                color = vec4(1.0);
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
        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        // We handle events very differently between targets

        #[cfg(feature = "window-glutin")]
        {
            use glutin::event::{Event, WindowEvent};
            use glutin::event_loop::ControlFlow;

            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Wait;
                match event {
                    Event::LoopDestroyed => {
                        println!("Event::LoopDestroyed!");
                        return;
                    }
                    Event::EventsCleared => {
                        println!("EventsCleared");
                        windowed_context.window().request_redraw();
                    }
                    Event::WindowEvent { ref event, .. } => match event {
                        WindowEvent::Resized(logical_size) => {
                            println!("WindowEvent::Resized: {:?}", logical_size);
                            let dpi_factor = windowed_context.window().hidpi_factor();
                            windowed_context.resize(logical_size.to_physical(dpi_factor));
                        }
                        WindowEvent::RedrawRequested => {
                            println!("WindowEvent::RedrawRequested");
                            gl.clear(glow::COLOR_BUFFER_BIT);
                            gl.draw_arrays(glow::TRIANGLES, 0, 3);
                            windowed_context.swap_buffers().unwrap();
                        }
                        WindowEvent::CloseRequested => {
                            println!("WindowEvent::CloseRequested");
                            gl.delete_program(program);
                            gl.delete_vertex_array(vertex_array);
                            *control_flow = ControlFlow::Exit
                        }
                        _ => (),
                    },
                    _ => (),
                }
            });
        }

        #[cfg(not(feature = "window-glutin"))]
        render_loop.run(move |running: &mut bool| {
            #[cfg(feature = "window-sdl2")]
            {
                for event in events_loop.poll_iter() {
                    match event {
                        sdl2::event::Event::Quit { .. } => *running = false,
                        _ => {}
                    }
                }
            }

            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.draw_arrays(glow::TRIANGLES, 0, 3);

            if !*running {
                gl.delete_program(program);
                gl.delete_vertex_array(vertex_array);
            }
        });
    }
}
