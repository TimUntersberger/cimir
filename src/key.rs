use winit::event::VirtualKeyCode;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Key {
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
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
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
    Backspace,
    Equals,
    Minus,
    Enter,
    Escape,
    Tab,
    Space,
    LControl,
    LAlt,
    LShift,
    LWin,
    RControl,
    RAlt,
    RShift,
    RWin
}

impl Into<char> for Key {
    fn into(self) -> char {
        match self {
            Self::A => 'a',
            Self::B => 'b',
            Self::C => 'c',
            Self::D => 'd',
            Self::E => 'e',
            Self::F => 'f',
            Self::G => 'g',
            Self::H => 'h',
            Self::I => 'i',
            Self::J => 'j',
            Self::K => 'k',
            Self::L => 'l',
            Self::M => 'm',
            Self::N => 'n',
            Self::O => 'o',
            Self::P => 'p',
            Self::Q => 'q',
            Self::R => 'r',
            Self::S => 's',
            Self::T => 't',
            Self::U => 'u',
            Self::V => 'v',
            Self::W => 'w',
            Self::X => 'x',
            Self::Y => 'u',
            Self::Z => 'z',
            Self::One => '1',
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
            Self::Zero => '0',
            Self::Space => ' ',
            Self::Tab => '\t',
            Self::Enter => '\n',
            Self::Equals => '=',
            Self::Minus => '-',
            Self::F1 
            | Self::F2
            | Self::F3
            | Self::F4
            | Self::F5
            | Self::F6
            | Self::F7
            | Self::F8
            | Self::F9
            | Self::F10
            | Self::F11
            | Self::F12
            | Self::Backspace
            | Self::Escape
            | Self::LControl
            | Self::LAlt
            | Self::LShift
            | Self::LWin
            | Self::RControl
            | Self::RAlt
            | Self::RShift
            | Self::RWin => 0 as char,
        }
    }
}

impl From<VirtualKeyCode> for Key {
    fn from(key: VirtualKeyCode) -> Self {
        match key {
           VirtualKeyCode::A => Self::A,
           VirtualKeyCode::B => Self::B,
           VirtualKeyCode::C => Self::C,
           VirtualKeyCode::D => Self::D,
           VirtualKeyCode::E => Self::E,
           VirtualKeyCode::F => Self::F,
           VirtualKeyCode::G => Self::G,
           VirtualKeyCode::H => Self::H,
           VirtualKeyCode::I => Self::I,
           VirtualKeyCode::J => Self::J,
           VirtualKeyCode::K => Self::K,
           VirtualKeyCode::L => Self::L,
           VirtualKeyCode::M => Self::M,
           VirtualKeyCode::N => Self::N,
           VirtualKeyCode::O => Self::O,
           VirtualKeyCode::P => Self::P,
           VirtualKeyCode::Q => Self::Q,
           VirtualKeyCode::R => Self::R,
           VirtualKeyCode::S => Self::S,
           VirtualKeyCode::T => Self::T,
           VirtualKeyCode::U => Self::U,
           VirtualKeyCode::V => Self::V,
           VirtualKeyCode::W => Self::W,
           VirtualKeyCode::X => Self::X,
           VirtualKeyCode::Y => Self::Y,
           VirtualKeyCode::Z => Self::Z,
           VirtualKeyCode::Key1 => Self::One,
           VirtualKeyCode::Key2 => Self::Two,
           VirtualKeyCode::Key3 => Self::Three,
           VirtualKeyCode::Key4 => Self::Four,
           VirtualKeyCode::Key5 => Self::Five,
           VirtualKeyCode::Key6 => Self::Six,
           VirtualKeyCode::Key7 => Self::Seven,
           VirtualKeyCode::Key8 => Self::Eight,
           VirtualKeyCode::Key9 => Self::Nine,
           VirtualKeyCode::Key0 => Self::Zero,
           VirtualKeyCode::F1 => Self::F1,
           VirtualKeyCode::F2 => Self::F2,
           VirtualKeyCode::F3 => Self::F3,
           VirtualKeyCode::F4 => Self::F4,
           VirtualKeyCode::F5 => Self::F5,
           VirtualKeyCode::F6 => Self::F6,
           VirtualKeyCode::F7 => Self::F7,
           VirtualKeyCode::F8 => Self::F8,
           VirtualKeyCode::F9 => Self::F9,
           VirtualKeyCode::F10 => Self::F10,
           VirtualKeyCode::F11 => Self::F11,
           VirtualKeyCode::F12 => Self::F12,
           VirtualKeyCode::Back => Self::Backspace,
           VirtualKeyCode::Return => Self::Enter,
           VirtualKeyCode::Escape => Self::Escape,
           VirtualKeyCode::Tab => Self::Tab,
           VirtualKeyCode::Space => Self::Space,
           VirtualKeyCode::Equals => Self::Equals,
           VirtualKeyCode::Minus => Self::Minus,
           VirtualKeyCode::LControl => Self::LControl,
           VirtualKeyCode::LAlt => Self::LAlt,
           VirtualKeyCode::LShift => Self::LShift,
           VirtualKeyCode::LWin => Self::LWin,
           VirtualKeyCode::RControl => Self::RControl,
           VirtualKeyCode::RAlt => Self::RAlt,
           VirtualKeyCode::RShift => Self::RShift,
           VirtualKeyCode::RWin => Self::RWin,
            x => todo!("{:?}", x)
        }
    }
}
