use std::cmp::{max, min};

use bracket_lib::prelude::{Point, RandomNumberGenerator, Rect};

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;
pub const NUM_TILES: i32 = MAP_WIDTH * MAP_HEIGHT;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    N,
    S,
    E,
    W,
    NW,
    NE,
    SW,
    SE,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]

pub enum Curve {
    Concave,
    Convex,
}

impl From<Direction> for Point {
    fn from(val: Direction) -> Self {
        match val {
            Direction::NW => Self::new(-1, -1),
            Direction::N => Self::new(0, -1),
            Direction::NE => Self::new(1, -1),
            Direction::W => Self::new(-1, 0),
            Direction::E => Self::new(1, 0),
            Direction::SW => Self::new(-1, 1),
            Direction::S => Self::new(0, 1),
            Direction::SE => Self::new(1, 1),
        }
    }
}

impl Direction {
    const fn opposite(self) -> Self {
        match self {
            Self::NW => Self::SE,
            Self::N => Self::S,
            Self::NE => Self::SW,
            Self::W => Self::E,
            Self::E => Self::W,
            Self::SW => Self::NE,
            Self::S => Self::N,
            Self::SE => Self::NW,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Wall(Curve, Direction),
    Floor,
    Void,
}

pub struct Map {
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![Tile::Void; NUM_TILES as usize],
        }
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    pub const fn in_bounds(point: Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && point.x < (MAP_WIDTH as i32)
            && point.y < (MAP_HEIGHT as i32)
    }

    pub fn at(&self, point: Point) -> Option<Tile> {
        if !Self::in_bounds(point) {
            return None;
        }

        Some(self.unsafe_at(point))
    }

    pub fn set(&mut self, point: Point, new_tile: Tile) {
        if let Some(tile) = self.borrow_mut_at(point) {
            *tile = new_tile;
        }
    }

    pub fn borrow_mut_at(&mut self, point: Point) -> Option<&'_ mut Tile> {
        if !Self::in_bounds(point) {
            return None;
        }

        Some(self.unsafe_borrow_mut_at(point))
    }

    #[allow(clippy::cast_sign_loss)]
    fn unsafe_borrow_mut_at(&mut self, point: Point) -> &mut Tile {
        &mut self.tiles[(point.x + point.y * MAP_WIDTH) as usize]
    }

    pub fn can_enter(&self, point: Point) -> bool {
        self.at(point).map_or(false, |tile| tile == Tile::Floor)
    }

    #[allow(clippy::cast_sign_loss)]
    fn unsafe_at(&self, point: Point) -> Tile {
        self.tiles[(point.x + point.y * MAP_WIDTH) as usize]
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    pub fn coordinate_iter(&self) -> impl Iterator<Item = (Point, Tile)> + '_ {
        self.tiles.iter().enumerate().map(|(index, tile)| {
            (
                Point::new((index as i32) % MAP_WIDTH, (index as i32) / MAP_WIDTH),
                *tile,
            )
        })
    }
}

pub struct Builder<'a> {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub max_rooms: usize,
    pub rng: &'a mut RandomNumberGenerator,
}

impl<'a> Builder<'a> {
    pub fn new(rng: &'a mut RandomNumberGenerator) -> Self {
        Self {
            map: Map::new(),
            rooms: vec![],
            max_rooms: 20,
            rng,
        }
    }

    pub fn build(mut self) -> Self {
        self.build_random_rooms();
        self.dig_random_tunnels();
        self.collapse_thin_vertical_walls();
        self.collapse_thin_horizontal_walls();
        self.build_walls();

        self
    }

    fn room_intersection(&self, new_room: Rect) -> bool {
        self.rooms.iter().any(|room| new_room.intersect(room))
    }

    fn new_room(&mut self) -> Rect {
        Rect::with_size(
            self.rng.range(1, MAP_WIDTH),
            self.rng.range(1, MAP_HEIGHT),
            self.rng.range(2, 10),
            self.rng.range(2, 10),
        )
    }

    const fn room_in_bounds(rect: &Rect) -> bool {
        rect.x1 > 0
            && rect.x1 < (MAP_WIDTH as i32)
            && rect.x2 > 0
            && rect.x2 < (MAP_WIDTH as i32)
            && rect.y1 > 0
            && rect.y1 < (MAP_HEIGHT as i32)
            && rect.y2 > 0
            && rect.y2 < (MAP_HEIGHT as i32)
    }

    fn try_dig_random_room(&mut self) {
        let new_room = self.new_room();

        if self.room_intersection(new_room) {
            // return;
        }

        if !Self::room_in_bounds(&new_room) {
            return;
        }

        new_room.for_each(|point| self.map.set(point, Tile::Floor));

        self.rooms.push(new_room);
    }

    fn build_random_rooms(&mut self) {
        while self.rooms.len() < self.max_rooms {
            self.try_dig_random_room();
        }
    }

    fn dig_vertical_tunnel(map: &mut Map, x: i32, y1: i32, y2: i32) {
        (min(y1, y2)..=max(y1, y2)).for_each(|y| map.set(Point { x, y }, Tile::Floor));
    }

    fn dig_horizontal_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
        (min(x1, x2)..=max(x1, x2)).for_each(|x| map.set(Point { x, y }, Tile::Floor));
    }

    fn dig_random_tunnels(&mut self) {
        self.rooms
            .sort_by(|r1, r2| r1.center().x.cmp(&r2.center().x));

        self.rooms
            .iter()
            .zip(self.rooms[1..].iter())
            .map(|(a, b)| (a.center(), b.center()))
            .for_each(|(prev, next)| {
                if self.rng.range(0, 2) == 1 {
                    Self::dig_horizontal_tunnel(&mut self.map, prev.x, next.x, prev.y);
                    Self::dig_vertical_tunnel(&mut self.map, next.x, prev.y, next.y);
                } else {
                    Self::dig_vertical_tunnel(&mut self.map, prev.x, prev.y, next.y);
                    Self::dig_horizontal_tunnel(&mut self.map, prev.x, next.x, next.y);
                }
            });
    }

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss
    )]
    fn collapse_thin_vertical_walls(&mut self) {
        let mut runners = vec![0; NUM_TILES as usize];

        for x in 0..MAP_WIDTH {
            let mut runner = 0;

            for y in 0..MAP_HEIGHT {
                match self.map.at(Point { x, y }) {
                    Some(Tile::Void) => {
                        runner += 1;
                        runners[(x + y * MAP_WIDTH) as usize] = runner;
                    }
                    Some(Tile::Floor) => {
                        runner = 0;
                        runners[(x + y * MAP_WIDTH) as usize] = runner;
                    }
                    _ => {}
                }
            }
        }

        for x in (0..MAP_WIDTH).rev() {
            let mut longest = 0;

            for y in (0..MAP_HEIGHT).rev() {
                let current = runners[(x + y * MAP_WIDTH) as usize];

                if current == 0 {
                    longest = 0;
                    continue;
                }

                if current > longest {
                    longest = current;
                }

                if longest < 4 {
                    self.map.set(Point { x, y }, Tile::Floor);
                }
            }
        }
    }

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss
    )]
    fn collapse_thin_horizontal_walls(&mut self) {
        let mut runners = vec![0; NUM_TILES as usize];

        for y in 0..MAP_HEIGHT {
            let mut runner = 0;

            for x in 0..MAP_WIDTH {
                match self.map.at(Point { x, y }) {
                    Some(Tile::Void) => {
                        runner += 1;
                        runners[(x + y * MAP_WIDTH) as usize] = runner;
                    }
                    Some(Tile::Floor) => {
                        runner = 0;
                        runners[(x + y * MAP_WIDTH) as usize] = runner;
                    }
                    _ => {}
                }
            }
        }

        for y in (0..MAP_HEIGHT).rev() {
            let mut longest = 0;

            for x in (0..MAP_WIDTH).rev() {
                let current = runners[(x + y * MAP_WIDTH) as usize];

                if current == 0 {
                    longest = 0;
                    continue;
                }

                if current > longest {
                    longest = current;
                }

                if longest < 2 {
                    self.map.set(Point { x, y }, Tile::Floor);
                }
            }
        }
    }

    const CONCAVE_WALLS: &[Direction; 8] = &[
        Direction::N,
        Direction::S,
        Direction::W,
        Direction::E,
        Direction::NE,
        Direction::NW,
        Direction::SE,
        Direction::SW,
    ];
    const CONVEX_CORNERS: &[[Direction; 3]] = &[
        [Direction::W, Direction::NW, Direction::N],
        [Direction::E, Direction::NE, Direction::N],
        [Direction::W, Direction::SW, Direction::S],
        [Direction::E, Direction::SE, Direction::S],
    ];

    fn build_walls(&mut self) {
        self.map
            .coordinate_iter()
            .filter(|(_, tile)| *tile == Tile::Void)
            .filter_map(|(center, _)| -> Option<(Point, Tile)> {
                let is_floor = |direction: &Direction| {
                    self.map.at(Point::from(*direction) + center) == Some(Tile::Floor)
                };

                for directions in Builder::CONVEX_CORNERS {
                    if directions.iter().all(is_floor) {
                        return Some((center, Tile::Wall(Curve::Convex, directions[1])));
                    }
                }

                for direction in Builder::CONCAVE_WALLS {
                    if is_floor(direction) {
                        return Some((center, Tile::Wall(Curve::Concave, direction.opposite())));
                    }
                }

                None
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(point, tile)| self.map.set(point, tile));
    }
}
