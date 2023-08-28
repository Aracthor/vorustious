#[cfg(debug_assertions)]
#[track_caller]
pub unsafe fn gl_check() {
    let error = gl::GetError();
    if error != 0 {
        let caller_location = std::panic::Location::caller();
        let caller_line_number = caller_location.line();
        panic!("GL error file {caller_location}, line {caller_line_number}: {:#X}", error);
    }
}

#[inline]
#[cfg(not(debug_assertions))]
pub unsafe fn gl_check() {}
