pub unsafe fn start_gl_context(width: i32, height: i32) {
    gl::Enable(gl::DEPTH_TEST);
    gl::Viewport(0, 0, width, height);
    gl::ClearColor(0.0, 0.0, 0.0, 1.0);
}

pub unsafe fn clear_gl_context() {
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
}
