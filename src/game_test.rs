#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_initial_position() {
        let game_state = GameState::new();
        assert_eq!(game_state.snake.len(), 3);
    }

    #[test]
    fn test_collision_detection() {
        let mut game_state = GameState::new();
        game_state.snake.push_front((5, 6)); // Simulate a collision
        assert_eq!(game_state.update().is_err(), true);
    }
}
