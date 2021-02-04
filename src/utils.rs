
// 定义各种颜色
pub type Color = [f32;4];

pub const RED:Color = [1.0, 0.0, 0.0, 1.0];
pub const GREEN:Color = [0.0, 1.0, 0.0, 1.0];
pub const BLUE:Color = [0.0, 0.0, 1.0, 1.0];
pub const WHITE:Color = [1.0; 4];
pub const BLACK:Color = [0.0,0.0,0.0,1.0];
pub const GREY:Color = [0.2, 0.2, 0.2, 1.0];


pub fn get_inner_color(c:Color) ->Color{
    [c[0]*0.8,c[1]*0.8,c[2]*0.8,c[3]]
}
pub fn get_inner_size(outer:[f64;4],border_size:f64) ->[f64;4]{
    [outer[0] + border_size ,outer[1] + border_size,
        outer[2] - border_size * 2.0 ,outer[3] - border_size* 2.0]
}