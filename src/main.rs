use std::time::Duration;
// use std::mem;
// use std::os::raw::c_void;

// use c_str_macro::c_str;
// use cgmath::perspective;
// use cgmath::prelude::SquareMatrix;

// use gl::types::{GLfloat, GLsizei, GLsizeiptr};

use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::pixels::Color; 

//mod shader;
//mod vertex;

//use shader::Shader;
//use vertex::Vertex;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 600;
// const FLOAT_NUM: usize = 3;
// const VERTEX_NUM: usize = 3;
// const BUF_SIZE: usize = FLOAT_NUM * VERTEX_NUM;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    {
        let gl_attribute = video_subsystem.gl_attr();
        gl_attribute.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attribute.set_context_version(3, 1);
        let (major, minor) = gl_attribute.context_version();
        println!("init OpenGL: version={}.{}", major, minor);
    }

    let window = video_subsystem
        .window("SDL", WINDOW_WIDTH, WINDOW_HEIGHT)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    // let gl_context = window.gl_create_context().unwrap();
    // gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    //let shader = Shader::new("rsc/shader/shader.vs", "rsc/shader/shader.fs");

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
