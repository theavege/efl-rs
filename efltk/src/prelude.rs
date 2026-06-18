

#[derive(Default)]
pub enum MouseSignal {
    #[default]
    DownLeft,
    DownRight,
    UpLeft,
    Move,
}

impl SignalExt for MouseSignal {
    fn to_str(&self) -> &str {
        match self {
            Self::DownLeft => "mouse,down,1",
            Self::DownRight => "mouse,down,3",
            Self::UpLeft => "mouse,up,1",
            Self::Move => "mouse,move",
        }
    }
}