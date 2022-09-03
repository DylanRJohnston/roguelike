use std::cmp::{max, min};

use rltk::{Point, RandomNumberGenerator, Rect};

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 50;
pub const NUM_TILES: usize = MAP_WIDTH * MAP_HEIGHT;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            tiles: vec![Tile::Wall; NUM_TILES.try_into().unwrap()],
        }
    }

    pub fn in_bounds(point: Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && point.x < (MAP_WIDTH as i32)
            && point.y < (MAP_HEIGHT as i32)
    }

    pub fn at(&self, point: Point) -> Option<Tile> {
        if !Map::in_bounds(point) {
            return None;
        }

        Some(self.unsafe_at(point))
    }

    pub fn set(&mut self, point: Point, new_tile: Tile) {
        self.borrow_mut_at(point).map(|tile| *tile = new_tile);
    }

    pub fn borrow_mut_at<'a>(&'a mut self, point: Point) -> Option<&'a mut Tile> {
        if !Map::in_bounds(point) {
            return None;
        }

        Some(self.unsafe_borrow_mut_at(point))
    }

    fn unsafe_borrow_mut_at(&mut self, point: Point) -> &mut Tile {
        &mut self.tiles[((point.x as usize) + (point.y as usize) * MAP_WIDTH)]
    }

    pub fn can_enter(&self, point: Point) -> bool {
        self.at(point).map_or(false, |tile| tile == Tile::Floor)
    }

    fn unsafe_at(&self, point: Point) -> Tile {
        self.tiles[((point.x as usize) + (point.y as usize) * MAP_WIDTH)]
    }

    pub fn coordinate_iter<'a>(&'a self) -> impl Iterator<Item = (Point, Tile)> + 'a {
        (0..MAP_WIDTH as i32)
            .into_iter()
            .flat_map(|x| {
                (0..MAP_HEIGHT as i32)
                    .into_iter()
                    .map(move |y| Point { x, y })
            })
            .map(|point| (point, self.unsafe_at(point)))
    }
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
    pub max_rooms: usize,
    pub rng: RandomNumberGenerator,
}

impl MapBuilder {
    fn room_intersection(&self, new_room: Rect) -> bool {
        for room in self.rooms.iter() {
            if new_room.intersect(room) {
                return true;
            }
        }
        return false;
    }

    fn new_room(&mut self) -> Rect {
        Rect::with_size(
            self.rng.range(1, MAP_WIDTH),
            self.rng.range(1, MAP_HEIGHT),
            self.rng.range(2, 10),
            self.rng.range(2, 10),
        )
    }

    fn room_in_bounds(rect: &Rect) -> bool {
        rect.x1 > 0
            && rect.x1 < (MAP_WIDTH as i32)
            && rect.x2 > 0
            && rect.x2 < (MAP_WIDTH as i32)
            && rect.y1 > 0
            && rect.y1 < (MAP_HEIGHT as i32)
            && rect.y2 > 0
            && rect.y2 < (MAP_HEIGHT as i32)
    }

    fn try_dig_random_room(&mut self) -> bool {
        let new_room = self.new_room();

        if self.room_intersection(new_room) {
            return false;
        }

        if !MapBuilder::room_in_bounds(&new_room) {
            return false;
        }

        new_room.for_each(|point| self.map.set(point, Tile::Floor));

        self.rooms.push(new_room);

        return true;
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
            .for_each(|(prev, next)| match self.rng.range(0, 2) {
                1 => {
                    MapBuilder::dig_horizontal_tunnel(&mut self.map, prev.x, next.x, prev.y);
                    MapBuilder::dig_vertical_tunnel(&mut self.map, next.x, prev.y, next.y);
                }
                _ => {
                    MapBuilder::dig_vertical_tunnel(&mut self.map, prev.x, prev.y, next.y);
                    MapBuilder::dig_horizontal_tunnel(&mut self.map, prev.x, next.x, next.y);
                }
            });
    }
}
