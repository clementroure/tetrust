use piston_window::{WindowSettings, PistonWindow};
use piston_window::{clear, rectangle};

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Tetris",
        (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0],
                        [0.0, 0.0, 100.0, 100.0],
                        c.transform, g);
        });
    }
}
