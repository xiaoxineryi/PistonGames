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

// 窗口类，用来初始化窗口
struct Window{
    x_size:u32,
    y_size:u32,
    pixels_size: u32,
    border_size: u32
}

impl Window{
    pub fn default() ->Self{
        Window{
            x_size: 30,
            y_size: 30,
            pixels_size: 25,
            border_size:2
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
    GAME_OVER
}


struct Game{
    background_color:Color,
    window_setting:Window,
    snake:Snake,
    time_since_move: Instant,
    state:State
}



impl Game{
    pub fn new(b:Color,w:Window)-> Self{
        Game{ background_color: b, window_setting: w,
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
        match args {
            Button::Keyboard(key) => { self.on_key(key); }
            _ => {},
        }
    }

    pub fn process(&mut self){
        // 当前只做移动
        // 如果还没有到移动时间，就直接返回
        if self.time_since_move.elapsed() < Duration::from_millis(MOVE_TIME) {
            return;
        }
        // 到了移动时间，就移动
        self.move_snake();
        self.time_since_move = Instant::now();
    }

    fn move_snake(&mut self){
        let suc= self.snake.snake_move(self.window_setting.x_size,self.window_setting.y_size);
    }

    pub fn render(&self,gl:&mut GlGraphics,r:RenderArgs){
        gl.draw(r.viewport(), |c, g| {
            let mut draw = |color:Color,rect:[f64;4]|{
                Rectangle::new(color).draw(rect,&c.draw_state,c.transform,g);
            };
            let pixel_size = self.window_setting.pixels_size as f64;
            let border_size = self.window_setting.border_size as f64;
            // 绘制地图
            for i in 0..self.window_setting.x_size {
                for j in 0..self.window_setting.y_size {

                    let outer =[pixel_size * i as f64, pixel_size * j as f64, pixel_size , pixel_size ];
                    let inner = get_inner_size(outer,border_size);
                    draw(GREY, outer);
                    draw(BLACK, inner);
                }
            }
            // 渲染蛇的部分
            self.snake.render(pixel_size,border_size,draw);

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
