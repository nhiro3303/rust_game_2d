use std::mem;
use std::os::raw::c_void;
use std::time::Duration;

use c_str_macro::c_str;
use cgmath::{perspective, vec3};
// use cgmath::prelude::SquareMatrix;

use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use imgui::im_str;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;

// use cgmath::num_traits::Float;
use std::f32;

mod shader;
mod vertex;

use shader::Shader;
use vertex::Vertex;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

const WINDOW_WIDTH: u32 = 900;
const WINDOW_HEIGHT: u32 = 480;
const FLOAT_NUM: usize = 3;
const VERTEX_NUM: usize = 36;
const BUF_SIZE: usize = FLOAT_NUM * VERTEX_NUM;

fn main() {
    // SDL本体の初期化
    let sdl_context = sdl2::init().unwrap();

    // ウィンドウやディスプレイの機能を担当するVideoSubsystem構造体を取得
    let video_subsystem = sdl_context.video().unwrap();

    // プロファイルの設定
    {
        // 使用するOpenGLのバージョン情報を指定し、OpenGLに対応したウィンドウを作成する
        // GLAttr構造体の取得
        let gl_attribute = video_subsystem.gl_attr();

        // Don't use deprecated OpenGL functions (OpenGLコンテキストのプロファイルを指定)
        gl_attribute.set_context_profile(sdl2::video::GLProfile::Core);

        // Set the OpenGL context version (OpenGLコンテキストのバージョンを指定 : OpenGL 3.2)
        gl_attribute.set_context_version(3, 2);
        let (major, minor) = gl_attribute.context_version();
        println!("init OpenGL: version={}.{}", major, minor);
    } // 変数 gl_attribute が自動で破棄されるように、ブロック{}でスコープを明示的に指定している

    // ウィンドウの初期化(WindowBuilder構造体はウィンドウの特性を指定するためのメソッドを持っており、
    // それらはメソッドチェーン方式で呼び出すことができる)
    let window = video_subsystem
        .window("SDL", WINDOW_WIDTH, WINDOW_HEIGHT) // WindowBuilder構造体を返すメソッド
        .opengl() // OpenGLを有効にする
        .position_centered() // ウィンドウをディスプレイの中央に配置する
        // .borderless() // ウィンドウのボーダーをなくす
        // .resizable() // ウィンドウのサイズを変更可能にする
        // .fullscreen() // ウィンドウをフルスクリーンにする
        // .set_window_flags(0) // 上に列挙したような特性をフラグで指定する
        .build() // 上に列挙した特性をもつウィンドウを作成する
        .unwrap();
    // let window = match video_subsystem
    //     .window("SDL", WINDOW_WIDTH, WINDOW_HEIGHT)
    //     .position_centered()
    //     .opengl()
    //     .build()
    //     { // エラー処理の例
    //         Ok(window)=>window,
    //         Err(err)=>panic!("failed to build window: {:?}", err),
    //     };

    // GLContext構造体の作成とOpenGL APIの読み込み
    let _gl_context = window.gl_create_context().unwrap(); // OpenGLコンテキストを作成する
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _); // OpenGL APIの関数ポインタを取得する

    let shader = Shader::new("rsc/shader/shader.vs", "rsc/shader/shader.fs");

    // set buffer
    #[rustfmt::skip]
    let buffer_array: [f32; BUF_SIZE] = [
        // 1
        0.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        1.0, 1.0, 0.0,

        0.0, 0.0, 0.0,
        1.0, 1.0, 0.0,
        1.0, 0.0, 0.0,

        // 2
        0.0, 0.0, 1.0,
        0.0, 0.0, 0.0,
        1.0, 0.0, 0.0,

        0.0, 0.0, 1.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 1.0,

        // 3
        0.0, 1.0, 1.0,
        0.0, 0.0, 1.0,
        1.0, 0.0, 1.0,

        0.0, 1.0, 1.0,
        1.0, 0.0, 1.0,
        1.0, 1.0, 1.0,

        // 4
        0.0, 1.0, 0.0,
        0.0, 1.0, 1.0,
        1.0, 1.0, 1.0,

        0.0, 1.0, 0.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 0.0,

        // 5
        1.0, 0.0, 1.0,
        1.0, 0.0, 0.0,
        1.0, 1.0, 0.0,

        1.0, 0.0, 1.0,
        1.0, 1.0, 0.0,
        1.0, 1.0, 1.0,

        // 6
        0.0, 1.0, 1.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 0.0,

        0.0, 1.0, 1.0,
        0.0, 0.0, 0.0,
        0.0, 0.0, 1.0,
    ];

    // キャンバスの取得と塗りつぶし
    // let mut canvas = window.into_canvas().build().unwrap();
    // canvas.set_draw_color(Color::RGB(8, 39, 245)); // 塗りつぶす色の指定する
    // canvas.clear(); // 指定した色で塗りつぶしてバッファーをクリアする
    // canvas.present(); // バッファーを切り替えて描画内容を画面に表示する

    let vertex = Vertex::new(
        (BUF_SIZE * mem::size_of::<GLfloat>()) as GLsizeiptr, // 頂点データのデータサイズ
        buffer_array.as_ptr() as *const c_void,               // 頂点データへのポインタ
        gl::STATIC_DRAW,                                      // 頂点データへのアクセス頻度
        vec![gl::FLOAT],        // 各頂点属性のデータ型を格納したベクター型
        vec![FLOAT_NUM as i32], // 各頂点属性のデータサイズを格納したベクター型
        FLOAT_NUM as i32 * mem::size_of::<GLfloat>() as GLsizei, // 各頂点データの始まりが何個おきに並んでいるか
        VERTEX_NUM as i32,                                       // 頂点の数
    );

    // init imgui
    let mut imgui_context = imgui::Context::create();
    // ウィジェットの位置などを保存する設定ファイルを作らない
    imgui_context.set_ini_filename(None);

    // init imgui sdl2
    let mut imgui_sdl2_context = imgui_sdl2::ImguiSdl2::new(&mut imgui_context, &window);
    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui_context, |s| {
        video_subsystem.gl_get_proc_address(s) as _
    });

    let mut depth_test: bool = true;
    let mut blend: bool = true;
    let mut wireframe: bool = true;
    let mut culling: bool = true;
    let mut camera_x: f32 = 3.0f32;
    let mut camera_y: f32 = -3.0f32;
    let mut camera_z: f32 = 3.0f32;

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut loops: i32 = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            imgui_sdl2_context.handle_event(&mut imgui_context, &event);
            if imgui_sdl2_context.ignore_event(&event) {
                continue;
            }
            // イベントキューにたまってるイベントをひとつづつ処理する
            match event {
                // 終了イベントかエスケープキーの押下イベントが発生したとき、runningラベルのついたループを抜ける
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // canvas.present();
        unsafe {
            // C言語由来の処理をunsafe{}で囲む
            if depth_test {
                gl::Enable(gl::DEPTH_TEST);
            } else {
                gl::Disable(gl::DEPTH_TEST);
            }

            if blend {
                gl::Enable(gl::BLEND);
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            } else {
                gl::Disable(gl::BLEND);
            }

            if wireframe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            } else {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }

            if culling {
                gl::Enable(gl::CULL_FACE);
            } else {
                gl::Disable(gl::CULL_FACE);
            }

            gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            // clear screen
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // COLOR_BUFFER_BIT : 描画する際にカラーバッファーを初期化する
                                                                    // DEPTH_BUFFER_BIT : 描画する際にデプスバッファーを初期化する(DEPTH_TESTを有効にするときは忘れずに！)

            // init matrice for model, view and projection
            // let model_matrix = Matrix4::identity();
            let buf: Vector3 = vec3( 0.5, 0.5, 0.0 );
            let model_matrix =
                Matrix4::from_translation(buf) * Matrix4::from_angle_z(cgmath::Rad(f32::consts::PI) * loops as f32 / 180f32) * Matrix4::from_translation(-buf);
            loops += 1;
            let view_matrix = Matrix4::look_at_rh(
                Point3 {
                    // 観測者の位置
                    x: camera_x,
                    y: camera_y,
                    z: camera_z,
                },
                Point3 {
                    // 見ているものの位置
                    x: 0.5,
                    y: 0.5,
                    z: 0.5,
                },
                Vector3 {
                    // 上下方向
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
            );
            let projection_matrix: Matrix4 = perspective(
                cgmath::Deg(45.0f32),
                WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
                0.1,
                100.0,
            );

            // shader use matrices (set_mat4メソッドで行列をユニフォーム変数としてシェーダーの中で使えるようにする)
            shader.use_program();
            shader.set_mat4(c_str!("uModel"), &model_matrix);
            shader.set_mat4(c_str!("uView"), &view_matrix);
            shader.set_mat4(c_str!("uProjection"), &projection_matrix);

            vertex.draw(); // OpenGLによる描画

            imgui_sdl2_context.prepare_frame(
                imgui_context.io_mut(),
                &window,
                &event_pump.mouse_state()
            );
            let ui = imgui_context.frame();
            imgui::Window::new(im_str!("Information"))
                .size([300.0, 300.0], imgui::Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(im_str!("OpenGL Test App ver0.1"));
                    ui.separator();
                    ui.text(im_str!("FPS: {:.1}", ui.io().framerate));
                    let display_size = ui.io().display_size;
                    ui.text(format!(
                            "Display Size: ({:.1}, {:.1})",
                            display_size[0], display_size[1]
                    ));
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                            "Mouse Position: ({:.1}, {:.1})",
                            mouse_pos[0], mouse_pos[1]
                    ));
                    ui.separator();
                    ui.checkbox(im_str!("Depth Test"), &mut depth_test);
                    ui.checkbox(im_str!("Blend"), &mut blend);
                    ui.checkbox(im_str!("Wireframe"), &mut wireframe);
                    ui.checkbox(im_str!("Culling"), &mut culling);
                    ui.separator();
                    #[rustfmt::skip]
                    imgui::Slider::new(im_str!("Camera X"))
                        .range(-5.0..=5.0)
                        .build(&ui, &mut camera_x);
                    #[rustfmt::skip]
                    imgui::Slider::new(im_str!("Camera Y"))
                        .range(-5.0..=5.0)
                        .build(&ui, &mut camera_y);
                    #[rustfmt::skip]
                    imgui::Slider::new(im_str!("Camera Z"))
                        .range(-5.0..=5.0)
                        .build(&ui, &mut camera_z);
                    ui.separator();
                    imgui::ProgressBar::new(0.6)
                        .size([200.0, 20.0])
                        .overlay_text(im_str!("Progress!"))
                        .build(&ui);
                    let arr = [0.6f32, 0.1f32, 1.0f32, 0.5f32, 0.92f32, 0.1f32, 0.2f32];
                    ui.plot_lines(im_str!("lines"), &arr)
                        .graph_size([200.0, 40.0])
                        .build();
                    ui.plot_histogram(im_str!("histogram"), &arr)
                        .graph_size([200.0, 40.0])
                        .build();
                });
            imgui_sdl2_context.prepare_render(&ui, &window);
            renderer.render(ui);

            window.gl_swap_window(); // 描画結果をウィンドウ上に表示
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // フレームレート : 60FPS
    }
}
