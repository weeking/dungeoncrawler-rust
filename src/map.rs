use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
    Exit,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point)
            && (self.tiles[map_ind(point.x, point.y)] == TileType::Floor
                || self.tiles[map_ind(point.x, point.y)] == TileType::Exit)
    }

    pub fn try_ind(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_ind(point.x, point.y))
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;

        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let ind = self.point2d_to_index(destination);
                Some(ind)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, ind: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(ind);

        if let Some(ind) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((ind, 1.0))
        }

        if let Some(ind) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((ind, 1.0))
        }

        if let Some(ind) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((ind, 1.0))
        }

        if let Some(ind) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((ind, 1.0))
        }

        exits
    }

    fn get_pathing_distance(&self, ind1: usize, ind2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(ind1), self.index_to_point2d(ind2))
    }

    fn is_opaque(&self, ind: usize) -> bool {
        self.tiles[ind as usize] != TileType::Floor
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}

pub fn map_ind(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
