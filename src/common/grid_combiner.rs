use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Default, Debug)]
pub struct GridPoint {
    pub x: i64,
    pub y: i64,
}

impl GridPoint {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Default, Debug)]
pub struct GridRect {
    from: GridPoint,
    to: GridPoint,
}

impl GridRect {
    pub fn to_position_size(&self) -> (Vec2, Vec2) {
        let middle_x = (self.from.x as f32 + self.to.x as f32) * 0.5;
        let middle_y = (self.from.y as f32 + self.to.y as f32) * 0.5;
        (
            Vec2::new(middle_x, middle_y),
            Vec2::new(self.width() as f32, self.height() as f32),
        )
    }

    fn from_point(point: GridPoint) -> Self {
        Self {
            from: point,
            to: point,
        }
    }

    fn row_above(&self) -> Self {
        Self {
            from: GridPoint::new(self.from.x, self.from.y - 1),
            to: GridPoint::new(self.to.x, self.from.y - 1),
        }
    }
    fn row_below(&self) -> Self {
        Self {
            from: GridPoint::new(self.from.x, self.to.y + 1),
            to: GridPoint::new(self.to.x, self.to.y + 1),
        }
    }
    fn col_left(&self) -> Self {
        Self {
            from: GridPoint::new(self.from.x - 1, self.from.y),
            to: GridPoint::new(self.from.x - 1, self.to.y),
        }
    }
    fn col_right(&self) -> Self {
        Self {
            from: GridPoint::new(self.to.x + 1, self.from.y),
            to: GridPoint::new(self.to.x + 1, self.to.y),
        }
    }

    fn overlaps(&self, other: GridRect) -> bool {
        self.from.x <= other.to.x
            && self.to.x >= other.from.x
            && self.from.y <= other.to.y
            && self.to.y >= other.from.y
    }

    fn width(&self) -> i64 {
        self.to.x - self.from.x + 1
    }

    fn height(&self) -> i64 {
        self.to.y - self.from.y + 1
    }

    fn size(&self) -> i64 {
        self.width() * self.height()
    }
}

#[derive(Clone, Default)]
pub struct GridCombiner {
    points: HashSet<GridPoint>,
}

impl GridCombiner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_point(&mut self, point: GridPoint) {
        self.points.insert(point);
    }

    pub fn points(&self) -> &HashSet<GridPoint> {
        &self.points
    }

    fn contains_rect(&self, rect: GridRect) -> bool {
        for x in rect.from.x..=rect.to.x {
            for y in rect.from.y..=rect.to.y {
                if !self.points.contains(&GridPoint::new(x, y)) {
                    return false;
                }
            }
        }
        true
    }

    fn remove_rect(&mut self, rect: GridRect) -> bool {
        for x in rect.from.x..=rect.to.x {
            for y in rect.from.y..=rect.to.y {
                self.points.remove(&GridPoint::new(x, y));
            }
        }
        true
    }

    fn find_largest_rects(&self) -> Vec<GridRect> {
        let mut rects = vec![];
        for point in self.points.iter() {
            let mut width_first_rect = GridRect::from_point(*point);
            while self.points.contains(&GridPoint::new(
                width_first_rect.from.x - 1,
                width_first_rect.from.y,
            )) {
                width_first_rect.from.x -= 1;
            }
            while self.points.contains(&GridPoint::new(
                width_first_rect.to.x + 1,
                width_first_rect.to.y,
            )) {
                width_first_rect.to.x += 1;
            }
            while self.contains_rect(width_first_rect.row_above()) {
                width_first_rect.from.y -= 1;
            }
            while self.contains_rect(width_first_rect.row_below()) {
                width_first_rect.to.y += 1;
            }
            let width_tuple = (width_first_rect, width_first_rect.size());
            if !rects.contains(&width_tuple) {
                rects.push(width_tuple);
            }

            let mut height_first_rect = GridRect::from_point(*point);
            while self.points.contains(&GridPoint::new(
                height_first_rect.from.x,
                height_first_rect.from.y - 1,
            )) {
                height_first_rect.from.y -= 1;
            }
            while self.points.contains(&GridPoint::new(
                height_first_rect.to.x,
                height_first_rect.to.y + 1,
            )) {
                height_first_rect.to.y += 1;
            }
            while self.contains_rect(height_first_rect.col_left()) {
                height_first_rect.from.x -= 1;
            }
            while self.contains_rect(height_first_rect.col_right()) {
                height_first_rect.to.x += 1;
            }
            let height_tuple = (height_first_rect, height_first_rect.size());
            if !rects.contains(&height_tuple) {
                rects.push(height_tuple);
            }
        }

        rects.sort_by(|a, b| a.1.cmp(&b.1));
        if rects.len() > 0 {
            let mut results: Vec<GridRect> = vec![];
            'outer: while let Some(candidate_rect) = rects.pop() {
                for other in results.iter() {
                    if other.overlaps(candidate_rect.0) {
                        continue 'outer;
                    }
                }
                results.push(candidate_rect.0);
            }
            results
        } else {
            vec![]
        }
    }

    pub fn combine(&self) -> Vec<GridRect> {
        let mut clone = self.clone();
        let mut rects = vec![];
        loop {
            let largest_rects = clone.find_largest_rects();
            if largest_rects.len() == 0 {
                break;
            }
            for rect in largest_rects.into_iter() {
                rects.push(rect);
                clone.remove_rect(rect);
            }
        }
        rects
    }
}
