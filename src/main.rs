use piston_window::*;
use piston_window::color::hex;
use piston_window::{text, Glyphs};
use rand::rngs::{ThreadRng};
use rand::seq::SliceRandom;
use std::thread::sleep;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{Source};
use std::vec::Vec;
use find_folder::Search;
use native_dialog::{MessageDialog, MessageType};
#[path = "const/tetros.rs"] mod tetros;
#[path = "const/colors.rs"] mod colors;

// Tetros
struct Block {
    color: [f32;4],
    scheme: [[[u32;4] ; 4]; 4],
    coord: [u8; 2],
    rot: usize, // replace by enum
}

impl Block {
    fn new(color: &str, scheme: [[[u32;4];4];4]) -> Block {
        Block { 
            color: hex(color), 
            scheme: scheme,
            coord: [6, 0], 
            rot: 0, 
        }
    }
}

fn init_window () -> PistonWindow {

    let width:u32 = 380;
    let height:u32 = 760;

    let window: PistonWindow = WindowSettings::new("Tetrust - score: 0", [width, height]).vsync(true).exit_on_esc(true).resizable(false).build().unwrap();

    return window;
}

fn main() {

    // game variables
    let mut score = 0;
    let mut rng = rand::thread_rng();

    // game states
    let mut isGameStarted = false;
    let mut isGameOver = false;

    // speed
    let mut frame:u32= 0;
    let mut speed:u32 = 20;

    // initial empty grid with 0 value
    let mut grid = [[0; 14]; 23];

    // current tetros (the one the user is controlling)
    let mut index = 0;

    // import constants tetros + colors
    let tetros_list = [tetros::tetros_I, tetros::tetros_O, tetros::tetros_T, tetros::tetros_L,tetros::tetros_J,tetros::tetros_S,tetros::tetros_Z];
    let colors_list = [colors::CYAN,colors::YELLOW,colors::PURPLE,colors::GREEN,colors::RED,colors::BLUE,colors::ORANGE];
    
    // list of tetros active in the game
    let mut tetros_arr: Vec<Block> = vec![];

    // generate 1st random bag
    add_block(&mut rng, colors_list, tetros_list, &mut tetros_arr, &mut index);

    // absolute path of assets folder (relative path crash if trying to run the .exe outside of the editor)
    let assets = Search::ParentsThenKids(3, 3)
    .for_folder("assets")
    .unwrap();

    // Play music
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(assets.join("audios/music.mp3")).unwrap());
    let _source = Decoder::new(file).unwrap();
    let source = _source.repeat_infinite();
    let mut sink = Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(0.1);
    sink.append(source);

    // wait for the audio to load
    sleep(Duration::from_millis(1800));

    // open a window for the game
    let mut window = init_window();
    window.set_position((610, 5));

    // load font
    let ref font = assets.join("fonts/FiraSans-Regular.ttf");
    let mut glyphs = window.load_font(font).unwrap();

    isGameStarted = true;

    // Main loop
    while let Some(e) = window.next() {
        // disable keys and pause if gameOver or GameIsNotStarted
        if(isGameStarted && !isGameOver){
            // keyboard events
            if let Some(Button::Keyboard(key)) = e.press_args() {
                
                    if key == Key::Right {

                        moveHorizontal(1, &mut tetros_arr, colors_list, tetros_list, &mut index, &mut grid, &mut window, &mut score, &mut rng, &e);
                    }
                    if key == Key::Left {

                        moveHorizontal(-1, &mut tetros_arr, colors_list, tetros_list, &mut index, &mut grid, &mut window, &mut score, &mut rng, &e);
                    }
                    if key == Key::Down {

                        frame=speed;
                    }
                    if (key == Key::Up || key == Key::Space) {

                        rotate(&mut tetros_arr, index, grid);
                    }
            };

            // Update what's displayed on the window
            render(&mut window, &e, &mut tetros_arr, &mut glyphs);

            frame+=1;
            if (frame >= speed && isGameOver == false) {

            moveVertical(&mut isGameOver, &mut sink, &mut tetros_arr, colors_list, tetros_list, &mut index, &mut grid, &mut window, &mut score, &mut rng, &e);

            frame=0;
            tetros_arr[index].coord[1] += 1;
            }
        }

        sleep(Duration::from_nanos(167000)); // 60 fps
    }
}

fn rotate(tetros_arr: &mut Vec<Block>, index: usize, grid: [[u32; 14]; 23]){

    let mut canRot = true;
    let mut _rot = 0;

    if tetros_arr[index].rot < 3{

        _rot=tetros_arr[index].rot+1;
    }
    else{

        _rot=0;
    }

    for i in 0..tetros_arr[index].scheme[0].len() {
        for j in 0..tetros_arr[index].scheme[0].len() {

            if(tetros_arr[index].scheme[_rot][j][i] == 1){

                if(grid[(tetros_arr[index].coord[1] + j as u8) as usize][(tetros_arr[index].coord[0] + i as u8)as usize] == 1 || grid[(tetros_arr[index].coord[1] + j as u8) as usize][(tetros_arr[index].coord[0] + i as u8)as usize] == 1){

                    canRot = false;
                }

                if((tetros_arr[index].coord[0] + i as u8) < 2 || (tetros_arr[index].coord[0] + i as u8) > 11 || (tetros_arr[index].coord[1] + j as u8) > 21){

                    canRot = false;
                }
            }
        }
    }

    if(canRot){

            tetros_arr[index].rot=_rot;    
    }
}

fn render(window: &mut PistonWindow, e: &Event, tetros_arr: &mut Vec<Block>, glyphs: &mut Glyphs){

    let blockSize:f64 = 38.0;

    window.draw_2d(e, |c, g, _| {

        // background 
        clear([0.05, 0.05, 0.05, 1.0], g);

        // Text
        // let transform = c.transform.trans(450.0, 100.0);
        // text(
        //     [1.0, 1.0, 1.0, 1.0], 
        //     32,
        //     "TEXT",
        //     glyphs,
        //      transform,
        //     g
        // );

        // empty tiles
        for i in 0..10 {
            for j in 0..20 {

                rectangle([0.1, 0.1, 0.1, 1.0], 
                    [i as f64 *blockSize, j as f64*blockSize, blockSize-1.0, blockSize-1.0], 
                    c.transform, g);
            }
        }
        // tetros        
        for n in 0..tetros_arr.len(){
            for i in 0..tetros_arr[n].scheme[0].len() {
                for j in 0..tetros_arr[n].scheme[0].len() {

                    if(tetros_arr[n].scheme[tetros_arr[n].rot][j][i] == 1){

                        rectangle(tetros_arr[n].color, 
                            [(tetros_arr[n].coord[0] as f64 *blockSize + i as f64 * blockSize - blockSize*2.0), (tetros_arr[n].coord[1] as f64 *blockSize + j as f64 * blockSize - blockSize*3.0), blockSize-1.0, blockSize-1.0], 
                            c.transform, g);    
                    }
                }
            }
        }

    });

}

fn moveVertical(isGameOver: &mut bool, sink: &mut Sink, tetros_arr: &mut Vec<Block>, colors_list: [&str; 7], tetros_list: [[[[u32;4];4];4];7], index: &mut usize, grid: &mut [[u32; 14]; 23], window: &mut PistonWindow, score: &mut u32, rng: &mut ThreadRng, e: &Event){

    for i in 0..tetros_arr[*index].scheme[0].len() {
        for j in 0..tetros_arr[*index].scheme[0].len() {

            if(tetros_arr[*index].scheme[tetros_arr[*index].rot][j][i] == 1){
          
                if((tetros_arr[*index].coord[1] + j as u8) > 21 || grid[(tetros_arr[*index].coord[1] + j as u8 + 1) as usize][(tetros_arr[*index].coord[0] + i as u8)as usize] == 1){

                    ////////
                
                        for i in 0..tetros_arr[*index].scheme[0].len() {
                            for j in 0..tetros_arr[*index].scheme[0].len() {
                
                                if(tetros_arr[*index].scheme[tetros_arr[*index].rot][j][i] == 1){
                
                                    grid[(tetros_arr[*index].coord[1] as f64 + j as f64)as usize][(tetros_arr[*index].coord[0] as f64 + i as f64)as usize] = 1;
                                }
                            }
                        }

                        // check if game over
                        if((tetros_arr[*index].coord[1] + j as u8) < 4){

                            game_over(&mut *isGameOver, &mut *sink, *score, &mut *window);
                        }
    
                        // check if a line is full
                        line_check(&mut *score, &mut *grid, &mut *window, &mut *tetros_arr, *index, &e);

                        // fall next block
                        add_block(&mut *rng, colors_list, tetros_list, &mut *tetros_arr, &mut *index);
                }
            }
        }
    }
}

fn moveHorizontal(dir:i32, tetros_arr: &mut Vec<Block>, colors_list: [&str; 7], tetros_list: [[[[u32;4];4];4];7], index: &mut usize, grid: &mut [[u32; 14]; 23], window: &mut PistonWindow, score: &mut u32, rng: &mut ThreadRng, e: &Event){

    let mut canMove = true;

    for i in 0..tetros_arr[*index].scheme[0].len() {
        for j in 0..tetros_arr[*index].scheme[0].len() {

            if(tetros_arr[*index].scheme[tetros_arr[*index].rot][j][i] == 1){

                if(grid[(tetros_arr[*index].coord[1] + j as u8) as usize][(tetros_arr[*index].coord[0] as i32 + i as i32 + dir) as usize] == 1){

                    for i in 0..tetros_arr[*index].scheme[0].len() {
                        for j in 0..tetros_arr[*index].scheme[0].len() {
            
                            if(tetros_arr[*index].scheme[tetros_arr[*index].rot][j][i] == 1){
            
                                grid[(tetros_arr[*index].coord[1] as f64 + j as f64)as usize][(tetros_arr[*index].coord[0] as f64 + i as f64)as usize] = 1;
                            }
                        }
                    }

                    // check if a line is full
                    line_check(&mut *score, &mut *grid, &mut *window, &mut *tetros_arr, *index, &e);

                    // fall next block
                    add_block(&mut *rng, colors_list, tetros_list, &mut *tetros_arr, &mut *index);
                }

                // check if can move left or right

                if(dir > 0) { // right 

                    if((tetros_arr[*index].coord[0] + i as u8) > 10){

                        canMove = false;
                    }
                }
                else{ // left

                    if((tetros_arr[*index].coord[0] + i as u8) < 3){

                        canMove = false;
                    }
                }
            }
        }
    }

    if(canMove==true){

        tetros_arr[*index].coord[0] = ((tetros_arr[*index].coord[0]) as i32 + dir) as u8;   
    }
}

fn game_over(isGameOver: &mut bool, sink: &mut Sink, score: u32, window: &mut PistonWindow){

        *isGameOver = true;
        sink.stop();

        // let file = BufReader::new(File::open("assets/gameover.wav").unwrap());
        // let source = Decoder::new(file).unwrap();
        // sink.append(source);

        let yes = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Game Over !")
        .set_text(&format!("Final Score: {score}\nDo you want to restart ?"))
        .show_confirm()
        .unwrap();
        if yes {
            //window.set_should_close(true);
            main();
        }
        else{
            std::process::exit(0);
        }
}

fn line_check(score: &mut u32, grid: &mut [[u32;14];23], window: &mut PistonWindow, tetros_arr: &mut Vec<Block>, index: usize, e: &Event) {

    // how much lines are full after the last placement
    let mut line_cleared = vec![];

    // iterate trough the grid
    for i in 3..23 {
        let mut line: Vec<u32> = vec![];
        for j in 2..12 {
            line.push(grid[i as usize][j as usize])
        }
        if !line.contains(&0) {

            println!("line full");
            line_cleared.push(i);
        }
    }
   
    displayGrid(grid);

    if(line_cleared.len() > 0) {

        for nb_line in 0..line_cleared.len() {

            let _grid = grid.clone();

            // update grid
            // clear line
            for x in 2..12{
                grid[line_cleared[nb_line]][x] = 0;
            }
            // move line y-1
            for x in 2..12{
                for y in 2..line_cleared[nb_line]+1{
                    grid[y][x] =  _grid[y-1][x]
                }
            }

            // update tetros display pos
            // clear tetros parts that are on the line
            for n in 0..index+1 {
                let rot = tetros_arr[n].rot;
                for x in 0..tetros_arr[n].scheme[0].len() {
                    for y in 0..tetros_arr[n].scheme[0].len() {
            
                        if(tetros_arr[n].scheme[tetros_arr[n].rot][y][x] == 1){

                            if (tetros_arr[n].coord[1]+y as u8 == line_cleared[nb_line] as u8 && tetros_arr[n].coord[1] > 0) {

                                tetros_arr[n].scheme[rot][y][x] = 0;
                            }
                        }
                    }
                }
            }

            //render(window, e, tetros_arr);
            //sleep(Duration::from_millis(1000));

            // move tetros y-1
            for n in 0..index+1 {
                let mut hasAlreadyMoved = false;
                for x in 0..tetros_arr[n].scheme[0].len() {
                    for y in 0..tetros_arr[n].scheme[0].len() {
            
                        if(tetros_arr[n].scheme[tetros_arr[n].rot][y][x] == 1){

                            if(tetros_arr[n].coord[1]+y as u8 <= line_cleared[nb_line] as u8&& tetros_arr[n].coord[1] > 0){

                                if(!hasAlreadyMoved){
                                    hasAlreadyMoved = true;
                                    tetros_arr[n].coord[1]+=1;
                                 }
                            }
                        }
                    }
                }
            }
        }
    }

    displayGrid(grid);

    if(line_cleared.len() == 1){

        *score += 100;
    }
    else if(line_cleared.len() == 2){

        *score += 300;
    }
    else if(line_cleared.len() == 3){
        
        *score += 500;
    }
    else if(line_cleared.len() >= 4){
        
        *score += 800;
    }
    *score+=10;
    window.set_title(format!("Tetrust - score: {}", score));
}

fn add_block(rng: &mut ThreadRng, colors_list: [&str; 7], tetros_list: [[[[u32;4];4];4];7], tetros_arr: &mut Vec<Block>, index: &mut usize){

    if(*index == 0 || *index == tetros_arr.len()-1){

        let mut nums: Vec<usize> = (0..=6).collect();
        nums.shuffle(&mut *rng);
        for num in nums {
            //println!("{}", num);
            tetros_arr.push(Block::new(colors_list[num], tetros_list[num]));
        }
    }

    *index+=1;
}

// for debug only
fn displayGrid(grid: &mut [[u32;14];23]){

    for i in 2..23 {
        for j in 2..12 {
            print!("{}", grid[i][j]);
        }
        println!();
    }
    println!();
}