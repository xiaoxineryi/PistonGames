mod snake;
mod utils;

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use glutin_window::GlutinWindow;
use piston::{WindowSettings, Event, RenderArgs};

use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::RenderEvent;

use opengl_graphics::{GlGraphics, OpenGL};
use graphics::Rectangle;
use crate::utils::*;
use crate::snake::Snake;


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



struct Game{
    background_color:Color,
    window_setting:Window,
    snake:Snake
}



impl Game{
    pub fn new(b:Color,w:Window)-> Self{
        Game{ background_color: b, window_setting: w, snake:Snake::default() }
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

    let game = Game::new(WHITE,w);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&mut gl, r);
        }
    }
}
