// https://github.com/PistonDevelopers/piston-examples/blob/master/examples/user_input/src/main.rs

use piston_window::*;
use rand::Rng;
use fps_clock::*;
use std::{thread,time};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

struct Block {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    let width:f64 = 300.0;
    let height:f64 = 600.0;
    
    let mut window: PistonWindow = WindowSettings::new("Tetrust", [width as u32, height as u32]).exit_on_esc(true).vsync(true).fullscreen(false).build().unwrap();

    let mut fps = fps_clock::FpsClock::new(60);

    let mut rng = rand::thread_rng();
    let _r: f32 = rng.gen_range(0.0..1.0);
    let _g: f32 = rng.gen_range(0.0..1.0);
    let _b: f32 = rng.gen_range(0.0..1.0);

    let y_default = -60.0;
    let x_default = 0.0;

    let mut block_width = 90.0;
    let mut block_height = 60.0;

    let mut y_pos:f64 = y_default;
    let mut x_pos:f64 = x_default;

    let mut i:u32= 0;
    let mut isGameStarted:bool = false;
    let mut nb:u32 = 1;

    // Play music
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("assets/music.mp3").unwrap());
    let _source = Decoder::new(file).unwrap();
    let source = _source.repeat_infinite();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(0.2);
    sink.append(source);

    let mut rot:u32 = 0;

    while let Some(e) = window.next() {

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Right {
                if x_pos < width-block_width {
                    x_pos+=30.0;
                }
            }
            if key == Key::Left {
                if x_pos > 0.0 {
                   x_pos-=30.0;
                }
            }
            if key == Key::Down {
                if(y_pos<height){
                   y_pos+=30.0;
                }
            }
            if (key == Key::Up || key == Key::Space) {
                if rot < 3{
                   rot+=1;
                }
                else{
                    rot = 0;
                }
            }
        };

        let mut block_0 = [
            [x_pos, y_pos, 30.0, 30.0], 
            [x_pos+30.0, y_pos, 30.0, 30.0],
            [x_pos+60.0, y_pos, 30.0, 30.0], 
            [x_pos+60.0, y_pos+30.0, 30.0, 30.0]
            ];
        let block_1 = [
            [x_pos+30.0, y_pos, 30.0, 30.0], 
            [x_pos+30.0, y_pos+30.0, 30.0, 30.0],
            [x_pos, y_pos+60.0, 30.0, 30.0], 
            [x_pos+30.0, y_pos+60.0, 30.0, 30.0]
            ];
        let mut block_2 = [
            [x_pos, y_pos, 30.0, 30.0], 
            [x_pos, y_pos+30.0, 30.0, 30.0],
            [x_pos+30.0, y_pos+30.0, 30.0, 30.0], 
            [x_pos+60.0, y_pos+30.0, 30.0, 30.0]
            ]; 
        if x_pos>=width-60.0{
        block_2 = [
            [x_pos-30.0, y_pos, 30.0, 30.0], 
            [x_pos-30.0, y_pos+30.0, 30.0, 30.0],
            [x_pos, y_pos+30.0, 30.0, 30.0], 
            [x_pos+30.0, y_pos+30.0, 30.0, 30.0]
            ];
        block_0 = [
            [x_pos-30.0, y_pos, 30.0, 30.0], 
            [x_pos, y_pos, 30.0, 30.0],
            [x_pos+30.0, y_pos, 30.0, 30.0], 
            [x_pos+30.0, y_pos+30.0, 30.0, 30.0]
            ];
        }
        else{
        block_2 = [
            [x_pos, y_pos, 30.0, 30.0], 
            [x_pos, y_pos+30.0, 30.0, 30.0],
            [x_pos+30.0, y_pos+30.0, 30.0, 30.0], 
            [x_pos+60.0, y_pos+30.0, 30.0, 30.0]
            ]; 
        block_0 = [
            [x_pos, y_pos, 30.0, 30.0], 
            [x_pos+30.0, y_pos, 30.0, 30.0],
            [x_pos+60.0, y_pos, 30.0, 30.0], 
            [x_pos+60.0, y_pos+30.0, 30.0, 30.0]
            ];
        }
        let block_3 = [
            [x_pos, y_pos+60.0, 30.0, 30.0], 
            [x_pos+30.0, y_pos, 30.0, 30.0],
            [x_pos, y_pos+30.0, 30.0, 30.0], 
            [x_pos, y_pos, 30.0, 30.0]
            ];

        window.draw_2d(&e, |c, g, _| {
            clear([0.05, 0.05, 0.05, 1.0], g);
                               
            if !isGameStarted {
                let timer = time::Duration::from_millis(2000);
                thread::sleep(timer);
                isGameStarted = true;
            }
            else{

                if y_pos < height {

                    for j in 0..4 {
                        if rot == 0{

                            block_width=90.0;
                            block_height=60.0;

                            rectangle([_r, _g, _b, 1.0], 
                                block_0[0], 
                                c.transform, g);
                            rectangle([_r, _g, _b, 1.0], 
                                block_0[1], 
                                c.transform, g);
                            rectangle([_r, _g, _b, 1.0], 
                                block_0[2], 
                                c.transform, g);
                            rectangle([_r, _g, _b, 1.0], 
                                block_0[3], 
                                c.transform, g);
                        }
                        else if rot == 1{

                            block_width=60.0;
                            block_height=90.0;

                            rectangle([_r, _g, _b, 1.0], 
                                block_1[0], 
                                c.transform, g);
                            rectangle([_r, _g, _b, 1.0], 
                                block_1[1], 
                                c.transform, g);
                            rectangle([_r, _g, _b, 1.0], 
                                block_1[2], 
                                c.transform, g);
                            rectangle([_r, _g, _b, 1.0], 
                                block_1[3], 
                                c.transform, g);
                        }
                        else if rot == 2{

                            block_width=90.0;
                            block_height=60.0;

                            rectangle([_r, _g, _b, 1.0], 
                                block_2[0], 
                                c.transform, g);
                            rectangle([_r, _g, _b, 1.0], 
                                block_2[1], 
                                c.transform, g);
                            rectangle([_r, _g, _b, 1.0], 
                                block_2[2], 
                                c.transform, g);
                            rectangle([_r, _g, _b, 1.0], 
                                block_2[3], 
                                c.transform, g);
                        }
                        else{

                            block_width=60.0;
                            block_height=90.0;

                            rectangle([_r, _g, _b, 1.0], 
                                block_3[0], 
                                c.transform, g);
                            rectangle([_r, _g, _b, 1.0], 
                                block_3[1], 
                                c.transform, g);
                            rectangle([_r, _g, _b, 1.0], 
                                block_3[2], 
                                c.transform, g);
                            rectangle([_r, _g, _b, 1.0], 
                                block_3[3], 
                                c.transform, g);
                        }
                    }
                }
                else{
                    y_pos = y_default;
                    x_pos = x_default;
                }
            }
        });

        i+=1;
        if i >= 10 {
           i=0;
           y_pos += 30.0;
        }

        fps.tick();
    }
}
