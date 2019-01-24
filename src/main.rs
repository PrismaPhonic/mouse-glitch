use enigo::*;

use rand::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut enigo = Enigo::new();
    loop {
        let mut rng = rand::thread_rng();
        let random_time = rng.gen_range(0, 20);
        let x = rng.gen_range(300, 2200);
        let y = rng.gen_range(150, 1400);
        sleep(Duration::new(random_time, 0));
        enigo.mouse_move_to(x, y);

        let random_time = rng.gen_range(0, 10);
        let x = rng.gen_range(300, 2200);
        let y = rng.gen_range(150, 1400);
        sleep(Duration::new(random_time, 0));
        enigo.mouse_move_to(x, y);
    }
}
