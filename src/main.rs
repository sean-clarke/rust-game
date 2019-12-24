use std::time::{Duration, Instant};

fn main() {
	const GAME_REFRESH_RATE: f64 = 1_f64 / 60_f64;
	const FRAME_REFRESH_RATE: f64 = 1_f64 / 60_f64;

	let (mut game_refresh, mut frame_refresh, mut timer_refresh) = (0_f64, 0_f64, 0_f64);
	let mut delta: Duration;
	let mut delta_sum = 0_f64;

	let mut timer = Instant::now();

	loop {
		delta = timer.elapsed();
		timer += delta;
		delta_sum += delta.as_secs_f64();

		if delta_sum >= game_refresh {
			game_refresh += GAME_REFRESH_RATE;
		}

		if delta_sum >= frame_refresh {
			frame_refresh += FRAME_REFRESH_RATE;
		}

		if delta_sum >= timer_refresh {
			timer_refresh += 1_f64;
		}
	}
}
