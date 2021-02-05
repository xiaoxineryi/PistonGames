
// 定义各种颜色

pub type Color = [f32;4];

pub const RED:Color = [1.0, 0.0, 0.0, 1.0];
pub const GREEN:Color = [0.0, 1.0, 0.0, 1.0];
pub const BLUE:Color = [0.0, 0.0, 1.0, 1.0];
pub const WHITE:Color = [1.0; 4];
pub const BLACK:Color = [0.0,0.0,0.0,1.0];
pub const GREY:Color = [0.2, 0.2, 0.2, 1.0];
pub const DARK:Color = [0.0,0.0,0.0,0.9];
// 定义游戏相关数据

pub const MOVE_TIME:u64 = 200;