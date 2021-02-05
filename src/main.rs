mod snake;
mod utils;
mod setting;

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use glutin_window::GlutinWindow;
use piston::{WindowSettings, Event, RenderArgs, PressEvent, Button, Key};

use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::RenderEvent;

use opengl_graphics::{GlGraphics, OpenGL};
use graphics::Rectangle;
use crate::utils::*;
use crate::snake::{Snake, Direction};
use std::time::{Instant, Duration};
use crate::setting::*;
use rand::Rng;

// 窗口类，用来初始化窗口
struct Window{
    x_size:i32,
    y_size:i32,
    pixels_size: i32,
    border_size: i32
}

impl Window{
    pub fn default() ->Self{
        Window{
            x_size: 30,
            y_size: 30,
            pixels_size: 12,
            border_size:1
        }
    }
    pub fn get_size(&self) ->[u32;2]{
        [   (self.x_size * self.pixels_size  ) as u32,
            (self.y_size * self.pixels_size ) as u32
        ]
    }
}

enum State{
    MOVING,
    EATING(i8,Instant),
    GameOver
}


struct Game{
    background_color:Color,
    window_setting:Window,
    food_pos:(i32,i32),
    food_color:Color,
    snake:Snake,
    time_since_move: Instant,
    state:State
}



impl Game{
    pub fn new(b:Color,w:Window)-> Self{
        Game{ background_color: b, window_setting: w,
            food_pos: (5, 5),
            food_color: GREEN,
            snake:Snake::default(), time_since_move: Instant::now(), state: State::MOVING }
    }


    // 按下按键控制对应的走向
    fn on_key(&mut self,k:&Key){
        match &mut self.state {
            State::MOVING=>{
                let movement = match k {
                    Key::Right => Some(Direction::RIGHT),
                    Key::Left => Some(Direction::LEFT),
                    Key::Up => Some(Direction::UP),
                    Key::Down => Some(Direction::DOWN),
                    _ => None
                };
                if let Some(d) = movement{
                    self.snake.handle_direction(d);
                }
            },
            _ =>{}
        }
    }


    pub fn on_press(&mut self, args: &Button) {
        match &mut self.state {
            State::MOVING =>{
                match args {
                    Button::Keyboard(key) => { self.on_key(key); }
                    _ => {},
                }
            },
            _ =>{}
        }
    }

    pub fn process(&mut self){
        match &self.state {
            State::MOVING=>{
                // 当前只做移动
                // 如果还没有到移动时间，就直接返回
                if self.time_since_move.elapsed() < Duration::from_millis(MOVE_TIME) {
                    return;
                }
                // 到了移动时间，就移动
                self.move_snake();
                self.time_since_move = Instant::now();
            },
            _ =>{

            }
        }

    }

    fn move_snake(&mut self){
        let suc= self.snake.snake_move(self.window_setting.x_size,self.window_setting.y_size,&mut self.food_pos);
        // 如果越界或者撞到身体，则游戏失败
        if !suc{
            self.state = State::GameOver;
        }
    }

    fn draw_map<F>(&self,c_in:Color,c_out:Color,mut draw:F)
    where F:FnMut(Color,[f64;4]){
        let pixel_size = self.window_setting.pixels_size as f64;
        let border_size = self.window_setting.border_size as f64;
        // 绘制地图
        for i in 0..self.window_setting.x_size {
            for j in 0..self.window_setting.y_size {

                let outer =[pixel_size * i as f64, pixel_size * j as f64, pixel_size , pixel_size ];
                let inner = get_inner_size(outer,border_size);
                draw(c_out, outer);
                draw(c_in, inner);
            }
        }
    }
    fn draw_food<F>(&self,mut draw:F)
        where F:FnMut(Color,[f64;4]){
        let pixel_size = self.window_setting.pixels_size as f64;
        let border_size = self.window_setting.border_size as f64;
        let outer = [pixel_size * self.food_pos.0 as f64,
                            pixel_size * self.food_pos.1 as f64,
                            pixel_size,pixel_size
        ];
        let inner = get_inner_size(outer,border_size);
        let inner_color = get_inner_color(self.food_color);
        draw(self.food_color,outer);
        draw(inner_color,inner);
    }

    pub fn render(&self,gl:&mut GlGraphics,r:RenderArgs){
        gl.draw(r.viewport(), |c, g| {
            let mut draw = |color:Color,rect:[f64;4]|{
                Rectangle::new(color).draw(rect,&c.draw_state,c.transform,g);
            };
            let pixel_size = self.window_setting.pixels_size as f64;
            let border_size = self.window_setting.border_size as f64;
            match &self.state {
                State::GameOver =>{
                    self.snake.render(pixel_size,border_size,&mut draw);
                    self.draw_food(&mut draw);
                    self.draw_map(DARK,DARK,&mut draw);
                },
                _ =>{
                    self.draw_map(BLACK,GREY,&mut draw);
                    self.snake.render(pixel_size,border_size,&mut draw);
                    self.draw_food(&mut draw);
                }
            }
        });
    }
}

fn main() {
    let opengl =OpenGL::V3_2;
    let w = Window::default();
    let settings =WindowSettings::new("snake",w.get_size()).exit_on_esc(true);
    let mut window:GlutinWindow =settings.build().expect("could not create windows");
    let mut gl =GlGraphics::new(opengl);

    let mut game = Game::new(WHITE, w);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        game.process();

        if let Some(r) = e.render_args() {
            game.render(&mut gl, r);
        }

        if let Some(args) =e.press_args(){
            game.on_press(&args);
        }
    }
}
