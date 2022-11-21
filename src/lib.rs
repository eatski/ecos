use rand::seq::SliceRandom;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

mod state;
// Import the `window.alert` function from the Web.
#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

#[wasm_bindgen]
pub fn calculate(current: JsValue) -> JsValue {
    let mut state : state::State = serde_wasm_bindgen::from_value(current).unwrap();

    let mut cloned_state = state.clone();
    let mut rng = rand::thread_rng();
    cloned_state.points.shuffle(&mut rng);
    let head = cloned_state.points.pop().unwrap();
    let valid_directions = DIRECTIONS.iter().filter(|d| {
        let new_point = match d {
            Direction::Up => state::Point { x: head.x, y: head.y - 1 },
            Direction::Down => state::Point { x: head.x, y: head.y + 1 },
            Direction::Left => state::Point { x: head.x - 1, y: head.y },
            Direction::Right => state::Point { x: head.x + 1, y: head.y },
        };
        !state.points.contains(&new_point)
    }).collect::<Vec<_>>();
    let direction = valid_directions.choose(&mut rng).unwrap();

    let new_point = match direction {
        Direction::Up => state::Point { x: head.x, y: head.y - 1 },
        Direction::Down => state::Point { x: head.x, y: head.y + 1 },
        Direction::Left => state::Point { x: head.x - 1, y: head.y },
        Direction::Right => state::Point { x: head.x + 1, y: head.y },
    };

    if !state.points.contains(&new_point) {
        state.points.push(new_point);
    }
    serde_wasm_bindgen::to_value(&state).unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
