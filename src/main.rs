use piston_window::*;
use fps_clock::*;
use piston_window::color::hex;
use std::{thread};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{Source};
#[path = "const/tetros.rs"] mod tetros;
#[path = "const/colors.rs"] mod colors;

struct Block {
    color: [f32;4],
    scheme: [[[u32;4] ; 4]; 4],
    coord: [u32; 2],
    rot: usize,
    active: bool,
}

impl Block {
    fn new(color: &str, scheme: [[[u32;4];4];4]) -> Block {
        Block { 
            color: hex(color), 
            scheme: scheme,
            coord: [0, 0], 
            rot: 0, 
            active: true, 
        }
    }
}

fn main() {

    let width:f64 = 300.0;
    let height:f64 = 600.0;
    
    let mut window: PistonWindow = WindowSettings::new("Tetrust", [width as u32, height as u32]).exit_on_esc(true).build().unwrap();
    let mut fps = FpsClock::new(60); 
    let mut isGameStarted:bool = false;

    let mut frame:u32= 0;
    let mut speed:u32 = 10;

    // initial empty grid
    let mut grid: [[u32; 10]; 20] = [[0; 10]; 20];

    let mut tetros = Block::new(colors::light_blue, tetros::tetros_I);
    let mut tetros_arr: Vec<Block> = vec![tetros];

    let mut index = 0;

    // Play music
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("assets/music.mp3").unwrap());
    let _source = Decoder::new(file).unwrap();
    let source = _source.repeat_infinite();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(0.2);
    sink.append(source);

    while let Some(e) = window.next() {

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Right {
                if tetros_arr[index].coord[0] < 10 {

                    tetros_arr[index].coord[0]+=1;

                    for i in 0..20 {
                        for j in 0..10 {
                           print!("{}", grid[i][j]);
                        }
                        println!();
                    }
                }
            }
            if key == Key::Left {

                if tetros_arr[index].coord[0] > 0 {

                    tetros_arr[index].coord[0]-=1;
                }
            }
            if key == Key::Down {

                if(tetros_arr[index].coord[1]<20){

                    tetros_arr[index].coord[1]+=1;
                }
            }
            if (key == Key::Up || key == Key::Space) {

                if tetros_arr[index].rot < 3{
                    tetros_arr[index].rot+=1;
                }
                else{
                    tetros_arr[index].rot = 0;
                }
            }
        };

        window.draw_2d(&e, |c, g, _| {
            // background color
            clear([0.05, 0.05, 0.05, 1.0], g);

            // draw tiles
            for i in 0..10 {
                for j in 0..20 {

                    rectangle([0.1, 0.1, 0.1, 1.0], 
                        [i as f64 *30.0, j as f64*30.0, 29.0, 29.0], 
                        c.transform, g);
                }
            }
                  
            // load before play
            if !isGameStarted {
                let timer = Duration::from_millis(2000);
                thread::sleep(timer);
                isGameStarted = true;
            }
            else{

                    for n in 0..tetros_arr.len(){
                        for i in 0..tetros_arr[n].scheme[0].len() {
                            for j in 0..tetros_arr[n].scheme[0].len() {

                                if(tetros_arr[n].scheme[tetros_arr[n].rot][i][j] == 1){

                                    rectangle(hex("33FFF8"), 
                                        [(tetros_arr[n].coord[0] as f64 *30.0 + i as f64 * 30.0), (tetros_arr[n].coord[1] as f64 *30.0 + j as f64 * 30.0 - 60.0), 29.0, 29.0], 
                                        c.transform, g);    
                                }
                            }
                        }
                    }

                    if tetros_arr[index].coord[1] > 18 {
    
                        // for i in 0..tetros_arr[index].scheme[0].len() {
                        //     for j in 0..tetros_arr[index].scheme[0].len() {
                
                        //         if(tetros_arr[index].scheme[tetros_arr[index].rot][i][j] == 1){
                
                        //             grid[(tetros_arr[index].coord[1] as f64 -4.0 - j as f64)as usize][(tetros_arr[index].coord[0] as f64 + i as f64)as usize] = 0;
                        //             grid[(tetros_arr[index].coord[1] as f64 - j as f64)as usize][(tetros_arr[index].coord[0] as f64 + i as f64)as usize] = 1;
                        //         }
                        //     }
                        // }
    
                        // for i in 0..20 {
                        //     for j in 0..10 {
                        //        print!("{}", grid[i][j]);
                        //     }
                        //     println!();
                        // }
                        // println!();
    
                        let tetros = Block::new(colors::light_blue, tetros::tetros_I);
                        tetros_arr.push(tetros);
                        index+=1;
                    }
                }
        });

        frame+=1;
        if frame >= speed {
           frame=0;
           tetros_arr[index].coord[1] += 1;
        }

        fps.tick();
    }
}
