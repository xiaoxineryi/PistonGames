use crate::utils::*;
use crate::setting::*;

// 移动的方向
#[derive(PartialOrd, PartialEq)]
pub enum Direction{
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction{
    pub fn to_array(&self) ->[i8;2]{
        match self {
            Direction::UP    =>   [0,-1],
            Direction::DOWN  =>   [0,1],
            Direction::LEFT  =>   [-1,0],
            Direction::RIGHT =>   [1,0]
        }
    }
}


pub struct Snake{
    head_color:Color,
    body_color:Color,
    direction:Direction,
    body:Vec<(i8,i8)>,
    head:(i8,i8)
}

impl Snake{
    pub fn default()->Self{
        Snake{
            head_color:RED,
            body_color: BLUE,
            direction: Direction::RIGHT,
            body: vec![(0,1),(0,2)],
            head: (0, 0)
        }
    }

    pub fn handle_direction(&mut self,d:Direction){
        if self.direction == d {
            return ;
        }
        let s_d = self.direction.to_array();
        let d_d = d.to_array();

        if s_d[0] + d_d[0] == 0 && s_d[1] + d_d [1] == 0 {
            return ;
        }else{
            self.direction = d;
        }
    }

    pub fn snake_move(&mut self,x:u32,y:u32) -> bool{
        let direction = self.direction.to_array();

        if self.head.0 + direction[0] > x as i8 || self.head.0 + direction[0] < 0
            || self.head.1 + direction[1] > y as i8 || self.head.1 + direction[1] < 0{
            return false;
        }
        // 如果有身体部分，就把头现在的位置放入身体，将最后一个去除
        if self.body.len() >0 {
            self.body.remove(0);
            self.body.push((self.head.0,self.head.1));
        }

        self.head.0 += direction[0];
        self.head.1 += direction[1];

        true
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