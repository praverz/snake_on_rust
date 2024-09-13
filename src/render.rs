use crate::game::GameState;

pub fn render(state: &GameState) {
    // Clear the screen
    print!("{}[2J", 27 as char);

    // Render the game state
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if state.snake.contains(&(x, y)) {
                print!("S ");
            } else if state.food == (x, y) {
                print!("F ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
    println!("Use arrow keys to move. Press 'q' to quit.");
}
