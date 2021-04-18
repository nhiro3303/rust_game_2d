use std::mem;
use std::os::raw::c_void;
use std::time::Duration;

use c_str_macro::c_str;
use cgmath::perspective;
// use cgmath::prelude::SquareMatrix;

use gl::types::{GLfloat, GLsizei, GLsizeiptr};

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

const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 600;
const FLOAT_NUM: usize = 3;
const VERTEX_NUM: usize = 3;
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
        // .resizeable() // ウィンドウのサイズを変更可能にする
        // .fullscreen() // ウィンドウをフルスクリーンにする
        // .set_window_flags(0) // 上に列挙したような特性をフラグで指定する
        .build() // 上に列挙した特性をもつウィンドウを作成する
        .unwrap();
    // let window = match video_subsystem
    //     .window("SDL", WINDOW_WIDTH, WINDOW_HEIGHT)
    //     .position_centered()
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
    let buffer_array: [f32;BUF_SIZE] = [
        -1.0, -1.0, 0.0,
        1.0, -1.0, 0.0,
        0.0, 1.0, 0.0,
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

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut loops: i32 = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
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
            gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            // clear screen
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT); // 描画する際にカラーバッファーを初期化する

            // init matrice for model, view and projection
            // let model_matrix = Matrix4::identity();
            let model_matrix =
                Matrix4::from_angle_y(cgmath::Rad(f32::consts::PI) * loops as f32 / 180f32);
            loops += 1;
            let view_matrix = Matrix4::look_at_rh(
                Point3 {
                    // 観測者の位置
                    x: 0.0,
                    y: 0.0,
                    z: 5.0,
                },
                Point3 {
                    // 見ているものの位置
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    // 上下方向
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
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

            window.gl_swap_window(); // 描画結果をウィンドウ上に表示
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // フレームレート : 60FPS
    }
}
