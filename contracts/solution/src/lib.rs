#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};

use engine::{Client as GameEngine, Direction, MapElement};

mod engine {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/soroban_game_engine.wasm"
    );
}

#[contract]
pub struct Solution;

#[contractimpl]
impl Solution {
    pub fn solve(env: Env, engine_id: Address) {
        let engine = GameEngine::new(&env, &engine_id);

        // YOUR CODE START
        let mut upgraded = false;
        'outer: loop {
            for (position, element) in engine.get_map().iter() {
                if engine.p_points() >= 100 {
                    break 'outer;
                }
                if MapElement::FuelPod == element && engine.p_fuel() > 100 {
                    continue;
                }

                let direction_x = Direction::Right;
                let step_x = position.0 - engine.p_pos().0;
                if step_x < 0 {
                    continue;
                }
                engine.p_turn(&direction_x);
                engine.p_move(&Some(step_x.unsigned_abs()));

                let direction_y = Direction::Down;
                let step_y = engine.p_pos().1 - position.1;
                if step_y < 0 {
                    continue;
                }
                engine.p_turn(&direction_y);
                engine.p_move(&Some(step_y.unsigned_abs()));

                match element {
                    MapElement::FuelPod => engine.p_harvest(),
                    MapElement::Asteroid => engine.p_shoot(),
                }

                if engine.p_points() >= 5 && upgraded == false {
                    engine.p_upgrade();
                    upgraded = true;
                }
            }

            engine.p_turn(&Direction::DownRight);
            engine.p_move(&Some(2));
        }

        // YOUR CODE END
    }
}

#[cfg(test)]
mod test;
