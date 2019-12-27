use std::{
	time::{Duration, Instant},
};
use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};

struct GameState {
	timer: i64,
}

fn init_game() -> GameState {
	GameState {
		timer: 600_i64
	}
}

fn main() {
	// Initialize Window
	let event_loop = EventLoop::new();
	let window = WindowBuilder::new().build(&event_loop).unwrap();

	// Initialize Game Loop
	const GAME_REFRESH_RATE: f64 = 1_f64 / 60_f64;
	const FRAME_REFRESH_RATE: f64 = 1_f64 / 60_f64;

	let (mut game_refresh, mut frame_refresh, mut timer_refresh) = (0_f64, 0_f64, 0_f64);
	let mut delta: Duration = Duration::new(0, 0);
	let mut delta_sum = 0_f64;

	let mut timer = Instant::now();

	// Initialize Game State
	let mut game_state: GameState = init_game();

	// Start Game Loop
	event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => *control_flow = ControlFlow::Poll,
        }

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
			game_state.timer -= 1;
			println!("time remaining {}", game_state.timer);
			timer_refresh += 1_f64;
		}
    });
}
