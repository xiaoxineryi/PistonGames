use crate::utils::*;

// 移动的方向
pub enum Direction{
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub struct Snake{
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
    pub fn render<F>(&self,pixel_size:f64,border_size:f64,mut draw:F)
        where F:FnMut(Color,[f64;4]){

        // 绘制蛇头
        let head_outer = [pixel_size * self.head.0 as f64,
            pixel_size * self.head.1 as f64,
            pixel_size,pixel_size
        ];
        let head_inner = get_inner_size(head_outer,border_size);
        let header_outer_color = self.head_color;
        let header_inner_color = get_inner_color(header_outer_color);
        draw(header_outer_color,head_outer);
        draw(header_inner_color,head_inner);
        // 绘制蛇身
        for pos in self.body.iter() {
            let (x,y) = *(pos);
            let body_outer = [pixel_size * x as f64, pixel_size * y as f64,
                pixel_size,pixel_size
            ];
            let body_inner = get_inner_size(body_outer,border_size);
            let body_outer_color = self.body_color;
            let body_inner_color = get_inner_color(body_outer_color);
            draw(body_outer_color,body_outer);
            draw(body_inner_color,body_inner);
        }
    }
}