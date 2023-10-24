use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::render::{WindowCanvas, TextureQuery};
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::ttf;

const GRID_X_SIZE: i32 = 40;
const GRID_Y_SIZE: i32 = 30;
const DOT_SIZE_IN_PXS: u32 = 20;
const FPS: u32 = 60;
const FONT_POINT_SIZE: u16 = 20;

#[derive(PartialEq)]
pub enum GameState { Playing, Paused, Dead }
#[derive(PartialEq)]
pub enum SnakeDirection { Up, Down, Right, Left }

#[derive(PartialEq)]
pub struct Point {pub x: i32, pub y: i32}

pub struct Renderer {canvas: WindowCanvas}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas: sdl2::render::Canvas<Window> = window
            .into_canvas()
            .index(find_driver().unwrap())
            .build()
            .map_err(|e| e.to_string())?;
        Ok(Renderer {canvas})
    }

    fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point {x, y} = point;
        self.canvas.fill_rect(Rect::new(
            x * DOT_SIZE_IN_PXS as i32,
            y * DOT_SIZE_IN_PXS as i32,
            DOT_SIZE_IN_PXS,
            DOT_SIZE_IN_PXS,
        ))?;
    
        Ok(())
    }

    pub fn draw(&mut self, context: &GameContext, font: &ttf::Font) -> Result<(), String> {
        self.draw_background(context);
        self.draw_snake(context)?;
        self.draw_food(context)?;
        self.draw_score(font,context.score)?;
        self.canvas.present();
    
        Ok(())
    }

    fn draw_score(&mut self, font: &ttf::Font, score: i32) -> Result<(), String> {
        let color: Color = Color::RGB(255, 255, 255);
        let text: String = format!("{}{}", "Score: ", score);
        let texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext> = self.canvas.texture_creator();
        let font_surface: sdl2::surface::Surface = font
            .render(&text)
            .solid(color)
            .map_err(|e| e.to_string())?;
        let font_texture = texture_creator
            .create_texture_from_surface(&font_surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery {width, height, ..} = font_texture.query();
        let font_rect = Rect::new(0, 0, width, height);
        self.canvas.copy(&font_texture, None, font_rect)?;

        Ok(())
    }

    fn draw_background(&mut self, context: &GameContext) {
        let color: Color = match context.state {
            GameState::Playing => Color::RGB(0, 0, 0),
            GameState::Paused | GameState::Dead => Color::RGB(30, 30, 30),
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_snake(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::GREEN);
        for point in &context.snake_position {
            self.draw_dot(point)?;
        }
        Ok(())
    }

    fn draw_food(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RED);
        self.draw_dot(&context.food)?;
        Ok(())
    }  
}

pub struct GameContext {
    pub snake_position: Vec<Point>,
    pub snake_direction: SnakeDirection,
    pub food: Point,
    pub state: GameState,
    pub run: bool,
    pub score: i32,
    pub grow: bool,
    pub frame_counter: i32,
    pub allow_move: bool,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            snake_position: vec![Point {x: 3, y: 3}],
            snake_direction: SnakeDirection::Right,
            state: GameState::Playing,
            food: Point {x: 5, y: 5},
            run: true,
            score: 0,
            grow: false,
            frame_counter: 0,
            allow_move: true,
        }
    }
    
    pub fn next_tick(&mut self) {
        if self.state == GameState::Paused {
            return;
        }
        if self.state == GameState::Dead {
            return;
        }
        let head_position: &Point = self.snake_position.first().unwrap();
        let next_head_position: Point = match self.snake_direction {
            SnakeDirection::Up => Point {x: head_position.x, y: head_position.y - 1},
            SnakeDirection::Down => Point {x: head_position.x, y: head_position.y + 1},
            SnakeDirection::Right => Point {x: head_position.x + 1, y: head_position.y},
            SnakeDirection::Left => Point {x: head_position.x - 1, y: head_position.y},
        };

        if next_head_position.x > (GRID_X_SIZE - 1) || next_head_position.x < 0 || next_head_position.y > (GRID_Y_SIZE - 1) || next_head_position.y < 0 || self.snake_position.contains(&next_head_position) {
            self.state = GameState::Dead;
        }

        self.grow = false;

        if next_head_position == self.food {
            self.eat_food();
            self.grow = true;
        }

        if !self.grow {
            self.snake_position.pop();
        }
        self.snake_position.reverse();
        self.snake_position.push(next_head_position);
        self.snake_position.reverse();
    }

    pub fn eat_food(&mut self) {
        self.score += 1;
        let x: u32 = rand::random::<u32>() % (GRID_X_SIZE as u32 - 1);
        let y: u32 = rand::random::<u32>() % (GRID_Y_SIZE as u32 - 1);
        self.food = Point {x: x as i32, y: y as i32};
    }

    pub fn process_input(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.close(),
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Up | Keycode::W => self.move_up(),
                        Keycode::Left | Keycode::A => self.move_left(),
                        Keycode::Down | Keycode::S => self.move_down(),
                        Keycode::Right | Keycode::D => self.move_right(),
                        Keycode::Escape => match self.state {
                            GameState::Playing => self.state = GameState::Paused,
                            GameState::Paused => self.close(),
                            GameState::Dead => self.close(),
                        },
                        Keycode::Space => match self.state {
                            GameState::Playing => {},
                            GameState::Paused => self.state = GameState::Playing,
                            GameState::Dead => self.restart(),
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        self.frame_counter += 1;
        if self.frame_counter % 3 == 0 {
            self.next_tick();
            self.frame_counter = 0;
            self.allow_move = true;
        }
    }

    pub fn move_up(&mut self) {
        if self.snake_direction != SnakeDirection::Down && self.state == GameState::Playing && self.allow_move {
            self.snake_direction = SnakeDirection::Up;
            self.allow_move = false;
        }
    }
    
    pub fn move_down(&mut self) {
        if self.snake_direction != SnakeDirection::Up && self.state == GameState::Playing && self.allow_move {
            self.snake_direction = SnakeDirection::Down;
            self.allow_move = false;
        }
    }
    
    pub fn move_right(&mut self) {
        if self.snake_direction != SnakeDirection::Left && self.state == GameState::Playing && self.allow_move {
            self.snake_direction = SnakeDirection::Right;
            self.allow_move = false;
        }
    }
    
    pub fn move_left(&mut self) {
        if self.snake_direction != SnakeDirection::Right && self.state == GameState::Playing && self.allow_move {
            self.snake_direction = SnakeDirection::Left;
            self.allow_move = false;
        }
    }

    pub fn close(&mut self) {
        self.run = false;
    }

    pub fn restart(&mut self) {
        self.snake_position = vec![Point {x: 3, y: 1}];
        self.snake_direction = SnakeDirection::Right;
        self.state = GameState::Playing;
        self.score = 0;
        self.food = Point {x: 3, y: 3};
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context: sdl2::Sdl = sdl2::init()?;
    let video_subsystem: sdl2::VideoSubsystem = sdl_context.video()?;
    let ttf_context: ttf::Sdl2TtfContext = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font: ttf::Font = ttf_context.load_font("assets/komikax_.ttf", FONT_POINT_SIZE)?;

    let window: Window = video_subsystem.window(
        "Snake",
        GRID_X_SIZE as u32 * DOT_SIZE_IN_PXS,
        GRID_Y_SIZE as u32 * DOT_SIZE_IN_PXS
    )
    .position_centered()
    .opengl()
    .build()
    .map_err(|e| e.to_string())?;

    let mut renderer: Renderer = Renderer::new(window)?;

    let mut event_pump: EventPump = sdl_context.event_pump()?;

    let mut context: GameContext = GameContext::new();



    while context.run {
        
        context.process_input(&mut event_pump);
        
        context.update();
    
        renderer.draw(&context, &font)?;
    }

    Ok(())
}

pub fn find_driver() -> Option<u32> {
    for (i, item) in  sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(i as u32);
        }
    }
    None
}
