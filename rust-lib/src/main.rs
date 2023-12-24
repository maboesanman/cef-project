
// extern "C" int main(int argc, char* argv[])

// use std::ffi::{c_int, c_void};
// extern "C" {
//     fn main_ffi(argc: c_int, argv: *const c_void) -> c_int;
// }

fn main() {
    println!("Hello, world!");
    // unsafe { main_ffi(0, std::ptr::null()) };
}
