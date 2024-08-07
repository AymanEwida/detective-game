use std::ptr;

use gl::types::*;

#[derive(Debug)]
pub struct Buffer {
    pub id: GLuint,
    target: GLuint
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.unbind();
            gl::DeleteBuffers(1, &self.id);
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

    pub unsafe fn unbind(&self) {
        gl::BindBuffer(self.target, 0);
    }

    pub unsafe fn set_empty(&self, size: usize, usage: GLuint) {
        self.bind();

        gl::BufferData(
            self.target,
            size as GLsizeiptr,
            ptr::null(),
            usage
        );
    }

    pub unsafe fn set_data<V>(&self, vertices_data: &[V], usage: GLuint) {
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
