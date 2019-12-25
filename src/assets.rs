use crate::constants::{MESH, INDICES};

pub fn init_buffers_from_constants<G: glow::HasContext>(gl: &G) -> <G as glow::HasContext>::VertexArray {
  unsafe {
    let vertex_array = gl
        .create_vertex_array()
        .expect("Cannot create vertex array");
    gl.bind_vertex_array(Some(vertex_array));

    let ibo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
    gl.buffer_data_size(
        glow::ELEMENT_ARRAY_BUFFER,
        INDICES.len() as i32 * std::mem::size_of::<u32>() as i32,
        glow::STATIC_DRAW,
    );
    gl.buffer_data_u8_slice(
        glow::ELEMENT_ARRAY_BUFFER,
        bytemuck::cast_slice(&INDICES),
        glow::STATIC_DRAW,
    );

    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_size(
        glow::ARRAY_BUFFER,
        MESH.len() as i32 * std::mem::size_of::<f32>() as i32,
        glow::STATIC_DRAW,
    );
    gl.buffer_data_u8_slice(
        glow::ARRAY_BUFFER,
        bytemuck::cast_slice(&MESH),
        glow::STATIC_DRAW,
    );

    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(
        0,
        3,
        glow::FLOAT,
        false,
        (6 * std::mem::size_of::<f32>()) as i32,
        0,
    );
    gl.enable_vertex_attrib_array(1);
    gl.vertex_attrib_pointer_f32(
        1,
        3,
        glow::FLOAT,
        false,
        (6 * std::mem::size_of::<f32>()) as i32,
        (3 * std::mem::size_of::<f32>()) as i32,
    );
    vertex_array
  }
}
