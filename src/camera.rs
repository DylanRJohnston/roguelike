use bracket_lib::prelude::Point;

#[derive(Debug, Default)]
pub struct Camera {
    width: i32,
    height: i32,

    left_x: i32,
    right_x: i32,
    top_y: i32,
    bottom_y: i32,
}

impl Camera {
    pub fn new(player_position: &Point, width: i32, height: i32) -> Self {
        let mut camera = Self {
            width,
            height,
            ..Default::default()
        };
        camera.update(player_position);

        camera
    }

    pub fn update(&mut self, point: &Point) {
        self.left_x = point.x - self.width / 2;
        self.right_x = point.x + self.width / 2;
        self.top_y = point.y - self.height / 2;
        self.bottom_y = point.y + self.height / 2;
    }

    pub fn worldspace_view_iter(&self) -> impl Iterator<Item = Point> + '_ {
        (self.left_x..self.right_x)
            .flat_map(|x| (self.top_y..=self.bottom_y).map(move |y| Point::new(x, y)))
    }

    pub fn to_camera_space(&self, Point { x, y }: &Point) -> Point {
        Point {
            x: x - self.left_x,
            y: y - self.top_y,
        }
    }
}
