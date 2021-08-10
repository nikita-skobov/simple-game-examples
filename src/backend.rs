use miniquad::{UserData, conf, EventHandler, Context, Pipeline, Bindings, Buffer, BufferType, Texture, Shader, BufferLayout, VertexAttribute, VertexFormat};

pub trait GameLoop {
    fn update(&mut self);
    fn draw(&mut self) -> TextureUpdate;
    fn init_canvas(&mut self, width: usize, height: usize) -> Vec<u8>;
}

pub trait Backend<T: GameLoop> {
    fn start(_game_loop: T) where Self: Sized + 'static {}
}

pub enum TextureUpdate {
    None,
    UpdateWhole(Vec<u8>),
    UpdatePart(i32, i32, i32, i32, Vec<u8>),
}


#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

pub struct MQBackend<T: GameLoop> {
    pipeline: Pipeline,
    bindings: Bindings,
    screen_width: u16,
    screen_height: u16,
    game_loop: T,
}

impl<T: GameLoop> EventHandler for MQBackend<T> {
    fn update(&mut self, _ctx: &mut Context) {
        self.game_loop.update();
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());
        let texture_update = self.game_loop.draw();
        match texture_update {
            TextureUpdate::None => {}
            TextureUpdate::UpdateWhole(new_pixels) => {
                let texture = Texture::from_rgba8(ctx, self.screen_width, self.screen_height, &new_pixels);
                self.bindings.images[0] = texture;
            }
            TextureUpdate::UpdatePart(x_offset, y_offset, width, height, pixel_slice) => {
                let texture = &mut self.bindings.images[0];
                texture.update_texture_part(ctx, x_offset, y_offset, width, height, &pixel_slice);
            }
        }
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.draw(0, 6, 1);
        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

impl<T: GameLoop> Backend<T> for MQBackend<T> {
    fn start(game_loop: T) where Self: Sized + 'static {
        miniquad::start(conf::Conf::default(), |mut ctx| {
            let init_obj = MQBackend::initialize(&mut ctx, game_loop);
            UserData::owning(init_obj, ctx)
        });
    }
}

impl<T: GameLoop> MQBackend<T> {
    fn initialize(ctx: &mut Context, game_loop: T,) -> MQBackend<T> {
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -1.0, y: -1.0 }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  1.0, y: -1.0 }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  1.0, y:  1.0 }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -1.0, y:  1.0 }, uv: Vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let (width, height) = ctx.screen_size();
        let (width, height) = (width as usize, height as usize);
        let mut game_loop = game_loop;
        let pixels = game_loop.init_canvas(width, height);
        let screen_width = width as u16;
        let screen_height = height as u16;
        let texture = Texture::from_rgba8(ctx, screen_width, screen_height, &pixels);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![texture],
        };

        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        MQBackend { pipeline, bindings, screen_width, screen_height, game_loop }
    }
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;
    uniform vec2 offset;
    varying lowp vec2 texcoord;
    void main() {
        gl_Position = vec4(pos + offset, 0, 1);
        texcoord = vec2(uv.s, 1.0 - uv.t);
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;
    uniform sampler2D tex;
    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub offset: (f32, f32),
    }
}
