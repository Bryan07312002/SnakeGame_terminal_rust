mod screen;
mod snake;
use screen::Screen;
use snake::Snake;

struct SnakeGame<'a>{
    pub screen: Screen<'a>,
    pub snake: Snake<'a>,
}
impl SnakeGame <'_>{
    pub fn new() -> SnakeGame <'static>{
        let screen = Screen::new();
        let center = screen.get_center();
        SnakeGame {
            screen: screen,
            snake: Snake::new(center)
        }
    }
    pub fn start() {
        
    }
}

fn main() {
    let game = SnakeGame::new();
    println!("{}",game.screen.screen);
}
