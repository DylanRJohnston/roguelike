use std::cmp::{max, min};

use bracket_lib::prelude::{Point, RandomNumberGenerator, Rect};

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;
pub const NUM_TILES: i32 = MAP_WIDTH * MAP_HEIGHT;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![Tile::Wall; NUM_TILES as usize],
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

    // pub fn coordinate_iter(&self) -> impl Iterator<Item = (Point, Tile)> + '_ {
    //     (0..MAP_WIDTH as i32)
    //         .into_iter()
    //         .flat_map(|x| {
    //             (0..MAP_HEIGHT as i32)
    //                 .into_iter()
    //                 .map(move |y| Point { x, y })
    //         })
    //         .map(|point| (point, self.unsafe_at(point)))
    // }
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

        if self.room_intersection(new_room) {}

        if !Self::room_in_bounds(&new_room) {}

        new_room.for_each(|point| self.map.set(point, Tile::Floor));

        self.rooms.push(new_room);
    }

    pub fn build_random_rooms(&mut self) {
        while self.rooms.len() < self.max_rooms {
            self.try_dig_random_room();
        }
    }

    fn dig_vertical_tunnel(map: &mut Map, x: i32, y1: i32, y2: i32) {
        (min(y1, y2)..max(y1, y2)).for_each(|y| map.set(Point { x, y }, Tile::Floor));
    }

    fn dig_horizontal_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
        (min(x1, x2)..max(x1, x2)).for_each(|x| map.set(Point { x, y }, Tile::Floor));
    }

    pub fn dig_random_tunnels(&mut self) {
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
}
