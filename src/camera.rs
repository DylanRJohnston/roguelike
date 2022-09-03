use rltk::Point;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(player_position: Point, width: i32, height: i32) -> Camera {
        Camera {
            left_x: player_position.x - width / 2,
            right_x: player_position.x + width / 2,
            top_y: player_position.y - height / 2,
            bottom_y: player_position.y + height / 2,
        }
    }

    pub fn worldspace_view_iter(&self) -> impl Iterator<Item = Point> + '_ {
        (self.left_x..self.right_x)
            .flat_map(|x| (self.top_y..self.bottom_y + 1).map(move |y| Point::new(x, y)))
    }

    pub fn to_camera_space(&self, Point { x, y }: &Point) -> Point {
        Point {
            x: x - self.left_x,
            y: y - self.top_y,
        }
    }
}
