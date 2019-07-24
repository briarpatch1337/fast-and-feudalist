use gl;
use std;
use std::ffi::{CString, CStr};
use filereader::{self, FileReader};

#[derive(Debug)]
pub enum Error {
    ResourceLoad { name: String, inner: filereader::Error },
    CanNotDetermineShaderTypeForResource { name: String },
    CompileError { name: String, message: String },
    LinkError { name: String, message: String },
}

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_file(gl: &gl::Gl, filereader: &FileReader, name: &str) -> Result<Program, Error> {
        const POSSIBLE_EXT: [&str; 2] = [
            ".vert",
            ".frag",
        ];

        let shaders = POSSIBLE_EXT.iter()
            .map(|file_extension| {
                Shader::from_file(gl, filereader, &format!("{}{}", name, file_extension))
            })
            .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(gl, &shaders[..]).map_err(|message| Error::LinkError {
            name: name.into(),
            message,
        })
    }

    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl.CreateProgram() };

        // glAttachShader
        for shader in shaders {
            unsafe { gl.AttachShader(program_id, shader.id()); }
        }

        // glLinkProgram
        unsafe { gl.LinkProgram(program_id); }

        // Check the link status of the program.
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            // A program linker error occurred.  Get more info from the compile log.

            // Get log length.
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            // convert buffer to CString
            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl.DetachShader(program_id, shader.id()); }
        }

        // The result of this expression is the return value
        Ok(Program { gl: gl.clone(), id: program_id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_file(gl: &gl::Gl, filereader: &FileReader, name: &str) -> Result<Shader, Error> {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] = [
        (".vert", gl::VERTEX_SHADER),
        (".frag", gl::FRAGMENT_SHADER),
    ];

    let shader_kind = POSSIBLE_EXT.iter()
        .find(|&&(file_extension, _)| {
            name.ends_with(file_extension)
            })
        .map(|&(_, kind)| kind)
        .ok_or_else(|| Error::CanNotDetermineShaderTypeForResource { name: name.into() })?;

    let source = filereader.load_cstring(name).map_err(|e| Error::ResourceLoad {
        name: name.into(),
        inner:e
    })?;

    Shader::from_source(gl, &source, shader_kind).map_err(|message| Error::CompileError {
        name: name.into(),
        message,
        })
    }

    pub fn from_source(
        gl: &gl::Gl,
        source: &CStr,
        kind: gl::types::GLenum
    ) -> Result<Shader, String> {

        // The ? at the end of this statement means that if shader_from_source returns an error,
    // this function will return an error.
        let id = shader_from_source(&gl, source, kind)?;

        Ok(Shader { gl: gl.clone(), id })
    }

    pub fn from_vert_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}


// Rust will ensure that glDeleteShader is called exactly once for every shader object id.
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

// helper function to compile a shader from string
fn shader_from_source(
    gl: &gl::Gl,  // a reference to gl
    source: &CStr,
    kind: gl::types::GLuint
) -> Result<gl::types::GLuint, String> {
    // glCreateShader
    // Create the shader
    let id = unsafe { gl.CreateShader(kind) };

    unsafe {
        // glShaderSource
        // Copy the source string for the shader program into the shader object.
        gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());

        // glCompileShader
        // Compile the shader source code.
        gl.CompileShader(id);
    }

    // Check the compile status of the shader program.
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        // A shader compile error occurred.  Get more info from the compile log.

        // Get log length.
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        // convert buffer to CString
        let info_log = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl.GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                info_log.as_ptr() as *mut gl::types::GLchar
            );
        }

        return Err(info_log.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
