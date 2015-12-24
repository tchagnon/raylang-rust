//! C library interface for creating Scenes and rendering them
extern crate image;
extern crate toml;
extern crate rustc_serialize;
extern crate libc;

mod color;
mod math;
mod mesh;
mod primitive;
mod ray_tracer;
mod scene;

use libc::c_char;
use std::path::Path;
use std::ffi::CStr;
use math::Vec3f;
use scene::Scene;

#[no_mangle]
pub extern fn vec3f(x: f32, y: f32, z: f32) -> *mut Vec3f {
    let v = Box::new(Vec3f::new(x, y, z));
    Box::into_raw(v)
}

#[no_mangle]
pub extern fn mag(v: *mut Vec3f) -> f32 {
    unsafe {
        (*v).magnitude()
    }
}

#[no_mangle]
pub extern fn read_scene(toml_path: *mut c_char) -> *const Scene {
    unsafe {
        let path_str = CStr::from_ptr(toml_path).to_str()
            .expect("Error converting path to string");
        let path = Path::new(path_str);
        let scene = Scene::read(path).prepare();
        let b = Box::new(scene);
        Box::into_raw(b)
    }
}

#[no_mangle]
pub extern fn render(scene: *const Scene) {
    unsafe {
        (*scene).render();
    }
}
