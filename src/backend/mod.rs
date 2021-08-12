pub mod b_miniquad;
pub use b_miniquad::*;

pub mod b_image;
pub use b_image::*;

/// copied from miniquad Conf and modified to remove
/// parts that arent relevant to us.
#[derive(Debug, Clone)]
pub struct BackendConf {
    /// Title of the window, defaults to an empty string.
    pub window_title: String,
    /// The preferred width of the window, ignored on wasm/android.
    ///
    /// Default: 800
    pub window_width: i32,
    /// The preferred height of the window, ignored on wasm/android.
    ///
    /// Default: 600
    pub window_height: i32,
    /// Whether the window should be created in fullscreen mode, ignored on wasm/android.
    ///
    /// Default: false
    pub fullscreen: bool,
    /// Determines if the application user can resize the window
    pub window_resizable: bool,
}

impl Default for BackendConf {
    fn default() -> BackendConf {
        BackendConf {
            window_title: "".to_owned(),
            window_width: 800,
            window_height: 600,
            fullscreen: false,
            window_resizable: true,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum MouseButton {
    Right,
    Left,
    Middle,
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct KeyMods {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[repr(u32)]
pub enum KeyCode {
    Space,
    Apostrophe,
    Comma,
    Minus,
    Period,
    Slash,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Semicolon,
    Equal,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftBracket,
    Backslash,
    RightBracket,
    GraveAccent,
    World1,
    World2,
    Escape,
    Enter,
    Tab,
    Backspace,
    Insert,
    Delete,
    Right,
    Left,
    Down,
    Up,
    PageUp,
    PageDown,
    Home,
    End,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDecimal,
    KpDivide,
    KpMultiply,
    KpSubtract,
    KpAdd,
    KpEnter,
    KpEqual,
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    Menu,
    Unknown,
}

impl From<u32> for KeyCode {
    fn from(code: u32) -> Self {
        match code {
            0 => KeyCode::Space,
            1 => KeyCode::Apostrophe,
            2 => KeyCode::Comma,
            3 => KeyCode::Minus,
            4 => KeyCode::Period,
            5 => KeyCode::Slash,
            6 => KeyCode::Key0,
            7 => KeyCode::Key1,
            8 => KeyCode::Key2,
            9 => KeyCode::Key3,
            10 => KeyCode::Key4,
            11 => KeyCode::Key5,
            12 => KeyCode::Key6,
            13 => KeyCode::Key7,
            14 => KeyCode::Key8,
            15 => KeyCode::Key9,
            16 => KeyCode::Semicolon,
            17 => KeyCode::Equal,
            18 => KeyCode::A,
            19 => KeyCode::B,
            20 => KeyCode::C,
            21 => KeyCode::D,
            22 => KeyCode::E,
            23 => KeyCode::F,
            24 => KeyCode::G,
            25 => KeyCode::H,
            26 => KeyCode::I,
            27 => KeyCode::J,
            28 => KeyCode::K,
            29 => KeyCode::L,
            30 => KeyCode::M,
            31 => KeyCode::N,
            32 => KeyCode::O,
            33 => KeyCode::P,
            34 => KeyCode::Q,
            35 => KeyCode::R,
            36 => KeyCode::S,
            37 => KeyCode::T,
            38 => KeyCode::U,
            39 => KeyCode::V,
            40 => KeyCode::W,
            41 => KeyCode::X,
            42 => KeyCode::Y,
            43 => KeyCode::Z,
            44 => KeyCode::LeftBracket,
            45 => KeyCode::Backslash,
            46 => KeyCode::RightBracket,
            47 => KeyCode::GraveAccent,
            48 => KeyCode::World1,
            49 => KeyCode::World2,
            50 => KeyCode::Escape,
            51 => KeyCode::Enter,
            52 => KeyCode::Tab,
            53 => KeyCode::Backspace,
            54 => KeyCode::Insert,
            55 => KeyCode::Delete,
            56 => KeyCode::Right,
            57 => KeyCode::Left,
            58 => KeyCode::Down,
            59 => KeyCode::Up,
            60 => KeyCode::PageUp,
            61 => KeyCode::PageDown,
            62 => KeyCode::Home,
            63 => KeyCode::End,
            64 => KeyCode::CapsLock,
            65 => KeyCode::ScrollLock,
            66 => KeyCode::NumLock,
            67 => KeyCode::PrintScreen,
            68 => KeyCode::Pause,
            69 => KeyCode::F1,
            70 => KeyCode::F2,
            71 => KeyCode::F3,
            72 => KeyCode::F4,
            73 => KeyCode::F5,
            74 => KeyCode::F6,
            75 => KeyCode::F7,
            76 => KeyCode::F8,
            77 => KeyCode::F9,
            78 => KeyCode::F10,
            79 => KeyCode::F11,
            80 => KeyCode::F12,
            81 => KeyCode::F13,
            82 => KeyCode::F14,
            83 => KeyCode::F15,
            84 => KeyCode::F16,
            85 => KeyCode::F17,
            86 => KeyCode::F18,
            87 => KeyCode::F19,
            88 => KeyCode::F20,
            89 => KeyCode::F21,
            90 => KeyCode::F22,
            91 => KeyCode::F23,
            92 => KeyCode::F24,
            93 => KeyCode::F25,
            94 => KeyCode::Kp0,
            95 => KeyCode::Kp1,
            96 => KeyCode::Kp2,
            97 => KeyCode::Kp3,
            98 => KeyCode::Kp4,
            99 => KeyCode::Kp5,
            100 => KeyCode::Kp6,
            101 => KeyCode::Kp7,
            102 => KeyCode::Kp8,
            103 => KeyCode::Kp9,
            104 => KeyCode::KpDecimal,
            105 => KeyCode::KpDivide,
            106 => KeyCode::KpMultiply,
            107 => KeyCode::KpSubtract,
            108 => KeyCode::KpAdd,
            109 => KeyCode::KpEnter,
            110 => KeyCode::KpEqual,
            111 => KeyCode::LeftShift,
            112 => KeyCode::LeftControl,
            113 => KeyCode::LeftAlt,
            114 => KeyCode::LeftSuper,
            115 => KeyCode::RightShift,
            116 => KeyCode::RightControl,
            117 => KeyCode::RightAlt,
            118 => KeyCode::RightSuper,
            119 => KeyCode::Menu,
            _ => KeyCode::Unknown,
        }
    }
}

#[derive(Debug)]
pub enum Event {
    MouseDown { button: MouseButton, x: f32, y: f32 },
    MouseUp { button: MouseButton, x: f32, y: f32 },
    MouseMove { x: f32, y: f32 },
    MouseScroll { up: bool },
    KeyDown { modifier: KeyMods, code: KeyCode, repeated: bool },
    KeyUp { modifier: KeyMods, code: KeyCode },
}

pub trait GameLoop {
    fn update(&mut self, events: Vec<Event>);
    fn draw(&mut self) -> TextureUpdate;
    fn init_canvas(&mut self, width: usize, height: usize, bpp: usize) -> Vec<u8>;
}

pub trait Backend<T: GameLoop> {
    fn start(_conf: BackendConf, _game_loop: T) where Self: Sized + 'static {}
    fn bytes_per_pixel() -> usize;
}

pub enum TextureUpdate<'a> {
    None,
    UpdateWhole(&'a [u8]),
    UpdatePart(i32, i32, i32, i32, Vec<u8>),
}
