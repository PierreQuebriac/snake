#[derive(Debug, Default, PartialEq, Clone)]
pub struct Coordinates {
    x: u16,
    y: u16,
}
impl Coordinates {
    pub fn new(x: u16, y: u16) -> Coordinates {
        Coordinates { x, y }
    }
}

#[derive(Debug, Default)]

pub enum CurrentState {
    Start,
    Pause,
    #[default]
    Idle,
}

#[derive(Debug, Default)]
pub struct App {
    pub score: u32,
    pub snake: std::vec::Vec<Coordinates>,
    pub state: CurrentState,
}

impl App {
    pub fn new() -> App {
        App {
            score: 0,
            snake: vec![
                Coordinates::new(5, 5),
                Coordinates::new(5, 6),
                Coordinates::new(6, 6),
            ],
            state: CurrentState::Idle,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new() {
        let app = App::new();
    }
}
