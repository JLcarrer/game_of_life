#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![windows_subsystem = "windows"]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;
const SIZE: usize = 1;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new((WIDTH * SIZE) as f64, (HEIGHT * SIZE) as f64);
        WindowBuilder::new()
            .with_title("Game of Life")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new((WIDTH * SIZE) as u32, (HEIGHT * SIZE) as u32, surface_texture)?
    };

    let mut cell_map: [[bool;HEIGHT];WIDTH]; 
    cell_map = generate();

    event_loop.run(move |event, _, control_flow| {
        //Draw the current frame
        if let Event::RedrawRequested(_) = event {
            draw(pixels.get_frame(), cell_map);
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        //Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            cell_map = update(cell_map);
            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}

//Draw grid with size
fn draw(frame: &mut [u8], cell_map: [[bool;HEIGHT];WIDTH]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let j = i / SIZE as usize;
        let x = (j % WIDTH as usize) as i16;
        let y = (j / (HEIGHT * SIZE) as usize) as i16;

        let color = cell_map[x as usize][y as usize] as u8 * 255;
        let rgba = [color, color, color, 0xff];

        pixel.copy_from_slice(&rgba);
    }
}

//Random grid
fn generate() -> [[bool;HEIGHT];WIDTH]{
    let mut cell_map: [[bool;HEIGHT];WIDTH] = [[false; HEIGHT]; WIDTH]; 
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            if rand::random(){
                cell_map[i][j] = true;
            }
        }
    }
    return cell_map;
}

//Run one cycle
fn update(cell_map: [[bool;HEIGHT];WIDTH]) -> [[bool;HEIGHT];WIDTH]{
    let mut new_cell_map: [[bool;HEIGHT];WIDTH] = [[false; HEIGHT]; WIDTH]; 
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let mut neighbours_count: u8 = 0;
            //Left Top
            if x > 0 && y < HEIGHT - 2{
                neighbours_count += cell_map[x - 1][y + 1] as u8;
            }
            if x == 0 && y < HEIGHT - 2{
                neighbours_count += cell_map[WIDTH - 1][y + 1] as u8;
            }
            if x > 0 && y == 0{
                neighbours_count += cell_map[x - 1][HEIGHT - 1] as u8;
            }
            if x == 0 && y == 0 {
                neighbours_count += cell_map[WIDTH - 1][HEIGHT - 1] as u8;
            }
            //Top
            if y < HEIGHT - 2{
                neighbours_count += cell_map[x][y + 1] as u8;
            }
            if y == 0{
                neighbours_count += cell_map[x][HEIGHT - 1] as u8;
            }
            //Right Top
            if x < WIDTH - 2 && y < HEIGHT - 2 {
                neighbours_count += cell_map[x + 1][y + 1] as u8;
            }
            if x == WIDTH - 1 && y < HEIGHT - 2 {
                neighbours_count += cell_map[0][y + 1] as u8;
            }
            if x < WIDTH - 2 && y == HEIGHT - 1 {
                neighbours_count += cell_map[x + 1][0] as u8;
            }
            if x == WIDTH - 1 && y == HEIGHT - 1 {
                neighbours_count += cell_map[0][0] as u8;
            }
            //Right
            if x < WIDTH - 2 {
                neighbours_count += cell_map[x + 1][y] as u8;
            }
            if x == WIDTH - 1 {
                neighbours_count += cell_map[0][y] as u8;
            }
            //Right Bottom
            if x < WIDTH - 2 && y > 0 {
                neighbours_count += cell_map[x + 1][y - 1] as u8;
            }
            if x == WIDTH - 1 && y > 0 {
                neighbours_count += cell_map[0][y - 1] as u8;
            }
            if x < WIDTH - 2 && y == 0 {
                neighbours_count += cell_map[x + 1][HEIGHT - 1] as u8;
            }
            if x == WIDTH - 1 && y == 0 {
                neighbours_count += cell_map[0][HEIGHT - 1] as u8;
            }
            //Bottom
            if y > 0 {
                neighbours_count += cell_map[x][y - 1] as u8;
            }
            if y == HEIGHT - 1 {
                neighbours_count += cell_map[x][0] as u8;
            }
            //Left Bottom
            if x > 0 && y > 0 {
                neighbours_count += cell_map[x - 1][y - 1] as u8;
            }
            if x == 0 && y > 0 {
                neighbours_count += cell_map[WIDTH - 1][y - 1] as u8;
            }
            if x > 0 && y == 0 {
                neighbours_count += cell_map[x - 1][HEIGHT - 1] as u8;
            }
            if x == 0 && y == 0 {
                neighbours_count += cell_map[WIDTH - 1][HEIGHT - 1] as u8;
            }
            //Left
            if x > 0 {
                neighbours_count += cell_map[x - 1][y] as u8;
            }
            if x == 0 {
                neighbours_count += cell_map[WIDTH - 1][y] as u8;
            }
            //Rules
            if neighbours_count == 3 {
                new_cell_map[x][y] = true;
            }
            else if neighbours_count == 2 {
                new_cell_map[x][y] = cell_map[x][y];
            }
            else{
                new_cell_map[x][y] = false;
            }
        }
    }
    return new_cell_map;
}