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



// 定义各种颜色
type Color = [f32;4];

const RED:Color = [1.0, 0.0, 0.0, 1.0];
const GREEN:Color = [0.0, 1.0, 0.0, 1.0];
const BLUE:Color = [0.0, 0.0, 1.0, 1.0];
const WHITE:Color = [1.0; 4];
const BLACK:Color = [0.0,0.0,0.0,1.0];
const GREY:Color = [0.2, 0.2, 0.2, 1.0];

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


// 移动的方向
enum Direction{
    UP,
    DOWN,
    LEFT,
    RIGHT
}

struct Snake{
    head_color:Color,
    body_color:Color,
    direction:Direction,
    body:Vec<(u8,u8)>,
    head:(u8,u8)
}

impl Snake{
    pub fn default()->Self{
        Snake{
            head_color:RED,
            body_color: BLUE,
            direction: Direction::DOWN,
            body: vec![(0,1),(0,2)],
            head: (0, 0)
        }
    }
}

struct Game{
    background_color:Color,
    window_setting:Window,
    snake:Snake
}

fn get_inner_color(c:Color) ->Color{
    [c[0]*0.8,c[1]*0.8,c[2]*0.8,c[3]]
}
fn get_inner_size(outer:[f64;4],border_size:f64) ->[f64;4]{
    [outer[0] + border_size ,outer[1] + border_size,
        outer[2] - border_size * 2.0 ,outer[3] - border_size* 2.0]
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
            // 绘制蛇头
            let head_outer = [pixel_size * self.snake.head.0 as f64,
                                     pixel_size * self.snake.head.1 as f64,
                                    pixel_size,pixel_size
            ];
            let head_inner = get_inner_size(head_outer,border_size);
            let header_outer_color = self.snake.head_color;
            let header_inner_color = get_inner_color(header_outer_color);
            draw(header_outer_color,head_outer);
            draw(header_inner_color,head_inner);
            // 绘制蛇身
            for pos in self.snake.body.iter() {
                let (x,y) = *(pos);
                let body_outer = [pixel_size * x as f64, pixel_size * y as f64,
                                        pixel_size,pixel_size
                ];
                let body_inner = get_inner_size(body_outer,border_size);
                let body_outer_color = self.snake.body_color;
                let body_inner_color = get_inner_color(body_outer_color);
                draw(body_outer_color,body_outer);
                draw(body_inner_color,body_inner);
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

    let game = Game::new(WHITE,w);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&mut gl, r);
        }
    }
}
