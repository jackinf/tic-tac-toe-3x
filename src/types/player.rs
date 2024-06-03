#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

impl Player {
    pub fn color(&self) -> bevy::prelude::Color {
        match self {
            Player::One => bevy::prelude::Color::rgb(0.0, 0.0, 1.0),
            Player::Two => bevy::prelude::Color::rgb(1.0, 0.0, 0.0),
        }
    }
}
