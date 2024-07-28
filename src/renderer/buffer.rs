use std::fmt::Debug;

use gl::types::*;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Buffer {
    pub id: GLuint,
    target: GLuint
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

impl Buffer {
    pub unsafe fn new(target: GLuint) -> Self {
        let mut id: GLuint = 0;
        gl::GenBuffers(1, &mut id);

        Self { id, target }
    }
}

impl Buffer {
    pub unsafe fn bind(&self) {
        gl::BindBuffer(self.target, self.id);
    }

    pub unsafe fn set_data<V: Debug + PartialEq>(&self, vertices_data: &[V], usage: GLuint) {
        self.bind();

        let (_, data_bytes, _) = vertices_data.align_to::<u8>();

        gl::BufferData(
            self.target,
            data_bytes.len() as GLsizeiptr,
            data_bytes.as_ptr() as *const _,
            usage
        );
    }
}
