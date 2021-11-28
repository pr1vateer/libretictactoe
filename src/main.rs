use std::env;
use std::path;
use std::collections::HashMap;
use ggez::event::{self, MouseButton };
use ggez::{GameResult, Context};
use ggez::graphics::{self, Color};
use ggez::conf;
use glam::*;

const CELL_SIZE: f32 = 128.0;
const CELLS_PER_SIDE: f32 = 3.0;
const SCREEN_WIDTH: f32 = CELL_SIZE * CELLS_PER_SIDE;
const SCREEN_HEIGHT: f32 = CELL_SIZE * CELLS_PER_SIDE;

struct GameState {
    cells: [i32; 9],
    draw_points: HashMap<i32, mint::Point2<f32>>,
    cross_img: graphics::Image,
    o_img: graphics::Image
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let cross_img = graphics::Image::new(ctx, "/cross.png")?;
        let o_img = graphics::Image::new(ctx, "/o.png")?;
        let mut draw_points = HashMap::new();

        //Init helper hashmap that specifies where to draw for a particular cell
        for i in 0..9 {
            draw_points.insert(i, get_point_by_cell(i as f32));
        }

        let cells_init = [1,0,0,0,2,0,0,0,0];

        let result = GameState { cells: cells_init, cross_img: cross_img, o_img: o_img, draw_points: draw_points };
        return Ok(result);
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> () {
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
        if button != MouseButton::Left {
            return;
        }

        let cell = get_cell(x, y);
        self.cells[cell as usize] = 1;
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);

        //Draw grid
        for x in (0..(SCREEN_WIDTH as u32)+1).step_by(CELL_SIZE as usize) {
            let _ = draw_line(ctx, mint::Point2{ x: x as f32, y: 0.0 }, mint::Point2{ x: x as f32, y: SCREEN_HEIGHT });
        }

        for y in (0..(SCREEN_WIDTH as u32)+1).step_by(CELL_SIZE as usize) {
            let _ = draw_line(ctx, mint::Point2{ x: 0.0, y: y as f32 }, mint::Point2{ x: SCREEN_WIDTH, y: y as f32 });
        }
        
        //Draw Xs and 0s
        for i in 0..self.cells.len() {
            if self.cells[i] == 0 {
                continue;
            }

            let point = self.draw_points[&(i as i32)];
            let img_to_draw = if self.cells[i] == 1 { &self.cross_img } else { &self.o_img };
            let _ = graphics::draw(ctx, img_to_draw, graphics::DrawParam::new().dest(point));
        }

        graphics::present(ctx)?;

        Ok(())
    }
}

fn draw_line(ctx: &mut Context, point_from: mint::Point2<f32>, point_to: mint::Point2<f32>) -> GameResult {
    let line = graphics::Mesh::new_line(
        ctx, 
        &[point_from, point_to], 
        4.0, 
        graphics::Color::BLACK)?;

    graphics::draw(ctx, &line, (mint::Point2{ x: 0.0, y:0.0 },))?;

    Ok(())
}

fn get_point_by_cell(cell: f32) -> mint::Point2<f32> {
    let cell_y = (cell / CELLS_PER_SIDE).floor();
    let cell_x = (cell % CELLS_PER_SIDE).floor();
    let x = cell_x * CELL_SIZE;
    let y = cell_y * CELL_SIZE;

    println!("For cell {} x is {} {} y is {} {}", cell, cell_x, x, cell_y, y);

    return mint::Point2{ x: x, y: y };
}

fn get_cell(x: f32, y: f32) -> i32 {
    let cell_y = (y / CELL_SIZE).floor();
    let cell_x = (x / CELL_SIZE).floor();
    let cell = cell_y * CELLS_PER_SIDE + cell_x;
    println!("{} {} {}", cell_y, cell_x, cell);

    return cell as i32;
}

fn get_resource_path() -> path::PathBuf {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    return resource_dir;
}

fn main() -> GameResult {
    let resource_path = get_resource_path();

    let cb = ggez::ContextBuilder::new("Tic-tac-toe", "")
        .add_resource_path(resource_path)
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .window_setup(conf::WindowSetup::default().title("Tic-tac-toe"));
    let (mut ctx, event_loop) = cb.build()?;

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state);
}
