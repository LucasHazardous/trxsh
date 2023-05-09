use ogl33::*;

pub struct VertexArray(pub GLuint);
impl VertexArray {
    pub fn new() -> Option<Self> {
        let mut vao = 0;
        unsafe { glGenVertexArrays(1, &mut vao) };
        if vao != 0 {
            Some(Self(vao))
        } else {
            None
        }
    }

    pub fn bind(&self) {
        unsafe { glBindVertexArray(self.0) }
    }

    pub fn clear_binding() {
        unsafe { glBindVertexArray(0) }
    }
}
