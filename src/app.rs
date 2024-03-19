#[derive(Debug, Default)]

pub struct Coordinates {
    x: u16,
    y: u16,
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
    score: u32,
    snake: std::vec::Vec<Coordinates>,
    state: CurrentState,
}

impl App {
    pub fn new() -> App {
        App {
            score: 0,
            snake: Vec::new(),
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
