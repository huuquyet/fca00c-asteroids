#![no_std]

use engine::{Client as GameEngine, Direction, MapElement};
use soroban_sdk::{contractimpl, BytesN, Env};

pub struct Solution;

mod engine {
    soroban_sdk::contractimport!(file = "../game_engine.wasm");
}

mod test;

#[contractimpl]
impl Solution {
    pub fn solve(env: Env, engine_id: BytesN<32>) {
        let engine = GameEngine::new(&env, &engine_id);

        // YOUR CODE START
        let mut upgraded = false;
        'outer: loop {
            for (position, element) in engine.get_map().iter().filter_map(Result::ok) {
                
                if engine.p_points() >= 100 {
                    break 'outer;
                }

                let mut direction_x = Direction::Right;
                let step_x = position.0 - engine.p_pos().0;
                if step_x < 0 {
                    direction_x = Direction::Left;
                }
                engine.p_turn(&direction_x);
                engine.p_move(&Some(step_x.unsigned_abs()));

                let mut direction_y = Direction::Up;
                let step_y = position.1 - engine.p_pos().1;
                if step_y < 0 {
                    direction_y = Direction::Down;
                }
                engine.p_turn(&direction_y);
                engine.p_move(&Some(step_y.unsigned_abs()));

                match element {
                    MapElement::FuelPod => engine.p_harvest(),
                    MapElement::Asteroid => engine.p_shoot(),
                }
            }
            if engine.get_map().is_empty() {
                engine.p_turn(&Direction::UpRight);
                engine.p_move(&Some(1));
            }
            if engine.p_points() >= 5 && upgraded == false {
                engine.p_upgrade();
                upgraded = true;
            }
        }
        
        // YOUR CODE END
    }
}
