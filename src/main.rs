use piston_window::{WindowSettings, PistonWindow};
use piston_window::{clear, rectangle};
#[derive(Default)]
struct Display {
    resolution: (u32, u32),
}

impl Display {
    fn get_resolution(&self) -> (u32, u32) {
        self.resolution
    }

    fn render(&self, window: &mut PistonWindow, e: &Event) {
        window.draw_2d(e, |c, g, _| { 
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0],
                        [0.0, 0.0, 100.0, 100.0],
                        c.transform, g);
        });
    }
}

fn main() {
    let display = Display {
        resolution: (640, 480),
    };

    let mut window: PistonWindow = WindowSettings::new(
        "Tetris",
        display.get_resolution())
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            display.render(&mut window, &e);
        }
    }
}
