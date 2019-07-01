use render_gl;

pub fn draw_triangle(gl: &gl::Gl, shader_program: &render_gl::Program) {
    // Activate this program
    shader_program.set_used();

    let triangle_width = 1_f32;
    let triangle_height = 3_f32.sqrt() / 2_f32;

    // Some vertices for our triangle
    let vertices: Vec<f32> = vec![
    //  positions                                                  colors
    //  x                      y                            z      r    g    b
        triangle_width/2_f32,  -triangle_height/3_f32,      0.0,   1.0, 0.0, 0.0,   // bottom right
        -triangle_width/2_f32, -triangle_height/3_f32,      0.0,   0.0, 1.0, 0.0,   // bottom left
        0.0,                   triangle_height/3_f32*2_f32, 0.0,   0.0, 0.0, 1.0    // top
    ];

    // Vertex buffer object (VBO)
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
    }

    // bind the newly created buffer to the GL_ARRAY_BUFFER target
    // Copy data to it
    // This data is accessible to the shader program
    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
    }

    // Vertex array object (VAO)
    // Describes how to interpret the data in vertices and converts it to inputs for our vertex shader
    // See vertex shader source to see how this is used by the program

    let mut vao: gl::types::GLuint = 0;

    unsafe {
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);

        // position attribute values
        gl.VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // not normalized (doesn't apply to floats anyways, only ints and bytes)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attribute values)
            std::ptr::null() // offset of the first component
        );
        gl.EnableVertexAttribArray(0); // enable attribute at index/location 0

        // color attribute values
        gl.VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );
        gl.EnableVertexAttribArray(1);
    }
    unsafe {
        gl.DrawArrays(
            gl::TRIANGLES, // mode
            0, // starting index in the enabled arrays
            3 // number of indices to be rendered
        );
        gl.DeleteVertexArrays(1, &mut vao);
        gl.DeleteBuffers(1, &mut vbo);
    }
}


pub fn draw_point(gl: &gl::Gl, shader_program: &render_gl::Program) {
    shader_program.set_used();
    let vertices: Vec<f32> = vec![
        0.0, 0.0, 0.0, 1.0, 1.0, 1.0
    ];
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
    }
    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
    }
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);
        gl.VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null()
        );
        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );
        gl.EnableVertexAttribArray(1);
    }
    unsafe {
        gl.DrawArrays(
            gl::POINTS, // mode
            0, // starting index in the enabled arrays
            1 // number of indices to be rendered
        );
        gl.DeleteVertexArrays(1, &mut vao);
        gl.DeleteBuffers(1, &mut vbo);
    }
}


pub struct ColorSpec {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub struct PositionSpec {
    pub x: f32,
    pub y: f32
}

pub struct HexagonSpec {
    pub color: ColorSpec,
    pub pos: PositionSpec,
    pub width: f32
}


pub fn draw_hexagon(gl: &gl::Gl, shader_program: &render_gl::Program, hex_spec: HexagonSpec) {
    shader_program.set_used();

    let hexagon_width = hex_spec.width;
    let hexagon_height =  hexagon_width * 3_f32.sqrt()/2.0;
    let x_pos = hex_spec.pos.x;
    let y_pos = hex_spec.pos.y;
    let r_color = hex_spec.color.r as f32 / 255.0;
    let g_color = hex_spec.color.g as f32 / 255.0;
    let b_color = hex_spec.color.b as f32 / 255.0;

    //TODO the positioning additions can move to the GPU.
    let vertices: Vec<f32> = vec![
        x_pos, y_pos, 0.0,                                          r_color, g_color, b_color,
        x_pos + hexagon_width/2.0, y_pos, 0.0,                      r_color, g_color, b_color,
        x_pos + hexagon_width/4.0, y_pos - hexagon_height/2.0, 0.0, r_color, g_color, b_color,
        x_pos - hexagon_width/4.0, y_pos - hexagon_height/2.0, 0.0, r_color, g_color, b_color,
        x_pos - hexagon_width/2.0, y_pos, 0.0,                      r_color, g_color, b_color,
        x_pos - hexagon_width/4.0, y_pos + hexagon_height/2.0, 0.0, r_color, g_color, b_color,
        x_pos + hexagon_width/4.0, y_pos + hexagon_height/2.0, 0.0, r_color, g_color, b_color,
        x_pos + hexagon_width/2.0, y_pos, 0.0,                      r_color, g_color, b_color,
    ];
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
    }
    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
    }
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);
        gl.VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null()
        );
        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );
        gl.EnableVertexAttribArray(1);
    }
    unsafe {
        gl.DrawArrays(
            gl::TRIANGLE_FAN, // mode
            0, // starting index in the enabled arrays
            8 // number of indices to be rendered
        );
        gl.DeleteVertexArrays(1, &mut vao);
        gl.DeleteBuffers(1, &mut vbo);
    }
}


pub fn draw_hexagon_outline(gl: &gl::Gl, shader_program: &render_gl::Program, hex_spec: HexagonSpec) {
    shader_program.set_used();

    let hexagon_width = hex_spec.width;
    let hexagon_height =  hexagon_width * 3_f32.sqrt()/2.0;
    let x_pos = hex_spec.pos.x;
    let y_pos = hex_spec.pos.y;
    let r_color = hex_spec.color.r as f32 / 255.0;
    let g_color = hex_spec.color.g as f32 / 255.0;
    let b_color = hex_spec.color.b as f32 / 255.0;

    //TODO the positioning additions can move to the GPU.
    let vertices: Vec<f32> = vec![
        x_pos + hexagon_width/2.0, y_pos, 0.0,                      r_color, g_color, b_color,
        x_pos + hexagon_width/4.0, y_pos - hexagon_height/2.0, 0.0, r_color, g_color, b_color,
        x_pos - hexagon_width/4.0, y_pos - hexagon_height/2.0, 0.0, r_color, g_color, b_color,
        x_pos - hexagon_width/2.0, y_pos, 0.0,                      r_color, g_color, b_color,
        x_pos - hexagon_width/4.0, y_pos + hexagon_height/2.0, 0.0, r_color, g_color, b_color,
        x_pos + hexagon_width/4.0, y_pos + hexagon_height/2.0, 0.0, r_color, g_color, b_color,
    ];
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
    }
    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
    }
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);
        gl.VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null()
        );
        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );
        gl.EnableVertexAttribArray(1);
    }
    unsafe {
        gl.LineWidth(3.0);
        gl.DrawArrays(
            gl::LINE_LOOP, // mode
            0, // starting index in the enabled arrays
            6 // number of indices to be rendered
        );
        gl.DeleteVertexArrays(1, &mut vao);
        gl.DeleteBuffers(1, &mut vbo);
    }
}


pub fn write_rotate_data(gl: &gl::Gl, shader_program: &render_gl::Program, rotation_angle: f32) {
    let transform_data = glm::mat4(
        rotation_angle.cos(), -rotation_angle.sin(), 0.0, 0.0,
        rotation_angle.sin(), rotation_angle.cos(), 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0);

    unsafe {
        let rotate_loc = gl.GetUniformLocation(shader_program.id(), std::ffi::CString::new("rotate").unwrap().as_ptr());
        gl.ProgramUniformMatrix4fv(shader_program.id(), rotate_loc, 1, gl::FALSE, transform_data.as_array().as_ptr() as *const gl::types::GLfloat);
    }
}


pub fn write_scale_data(gl: &gl::Gl, shader_program: &render_gl::Program, aspect_ratio: f32) {
    // aspect_ratio is W/H
    let mut x_scale = 1_f32;
    let mut y_scale = 1_f32;
    if aspect_ratio >= 1.0 {
        x_scale = 1.0 / aspect_ratio;
    } else {
        y_scale = aspect_ratio;
    }

    let transform_data = glm::vec4(
        x_scale,
        y_scale,
        1.0,
        1.0);

    unsafe {
        let scale_loc = gl.GetUniformLocation(shader_program.id(), std::ffi::CString::new("scale").unwrap().as_ptr());
        gl.ProgramUniform4fv(shader_program.id(), scale_loc, 1, transform_data.as_array().as_ptr() as *const gl::types::GLfloat);
    }
}
