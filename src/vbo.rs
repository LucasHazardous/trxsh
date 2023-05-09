use ogl33::*;

pub struct Buffer(pub GLuint, pub GLenum);
impl Buffer {
    pub fn new(ty: GLenum) -> Option<Self> {
        let mut vbo = 0;
        unsafe {
            glGenBuffers(1, &mut vbo);
        }
        if vbo != 0 {
            Some(Self(vbo, ty))
        } else {
            None
        }
    }

    pub fn bind(&self) {
        unsafe { glBindBuffer(self.1, self.0) }
    }

    pub fn clear_binding(&self) {
        unsafe { glBindBuffer(self.1, 0) }
    }

    pub fn buffer_data(&self, data: &[u8]) {
        unsafe {
            glBufferData(
                self.1,
                data.len().try_into().unwrap(),
                data.as_ptr().cast(),
                GL_DYNAMIC_DRAW,
            );
        }
    }

    pub fn overwrite(&self, data: &[u8]) {
        unsafe {
            glBufferSubData(
                self.1,
                0,
                data.len().try_into().unwrap(),
                data.as_ptr().cast(),
            );
        }
    }
}
