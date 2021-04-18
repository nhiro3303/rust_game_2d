use std::mem;
use std::os::raw::c_void;

use gl::types::{GLenum, GLfloat, GLint, GLsizei, GLsizeiptr};

pub struct Vertex {
    vao: u32,
    _vbo: u32,
    vertex_num: i32,
}

impl Vertex {
    pub fn new(
        size: GLsizeiptr,
        data: *const c_void,
        usage: GLenum,
        attribute_type_vec: std::vec::Vec<GLenum>,
        attribute_size_vec: std::vec::Vec<GLint>,
        stride: GLsizei,
        vertex_num: i32,
    ) -> Vertex {
        // VAOとVBOを一意に紐づけるIDを格納する変数 vao, vbo
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            // OpenGLのコードがC言語でできているため、明示的にunsafe{}で囲んでいる
            // create vertex array object and vertex buffer object
            gl::GenVertexArrays(1, &mut vao); // GPU上にVAO用のメモリを1つ確保
            gl::GenBuffers(1, &mut vbo); // GPU上にVBO用のメモリを1つ確保

            // bind buffer (これから使用するVAOとVBOを指定する)
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size, data, usage); // VBOへのはじめてのデータ転送
                                                                 // (バッファーの種類、バッファーのサイズ、転送元のデータ、アクセス頻度を指定)

            let mut offset = 0;
            for i in 0..attribute_type_vec.len() {
                gl::EnableVertexAttribArray(i as u32); // i番目の頂点属性の配列を有効にする
                gl::VertexAttribPointer(
                    // GPUへ送る頂点属性のデータがどのようなまとまりになっているかを設定する
                    i as u32,              // 頂点属性の順番(0から始まる)
                    attribute_size_vec[i], // 頂点属性あたりの要素数
                    attribute_type_vec[i], // データ型
                    gl::FALSE,             // 整数を浮動小数点型に正規化するかどうか
                    stride,                // 各頂点データの始まりが何個おきに並んでいるのか
                    (offset * mem::size_of::<GLfloat>()) as *const c_void, // 頂点データの開始地点のオフセット
                );
                offset += attribute_size_vec[i] as usize;
            }

            // unbind (VAOとVBOを準備した後の片づけ: 空のIDをバインドしてVAOとVBOの紐づけを解除)
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Vertex {
            vao: vao,
            _vbo: vbo,
            vertex_num: vertex_num,
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao); // 再びVAOを紐づける
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_num); // 描画するプリミティブの種類、頂点データの開始インデックス、描画する頂点の数
            gl::BindVertexArray(0); // VAOの紐づけを解除
        }
    }
}
