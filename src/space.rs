/*
 * Copyright © 2024 Gregory P. Moyer
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::cmp;
use std::collections::BTreeSet;
use std::num::TryFromIntError;

/// Defines a container in two- or three-dimensional space.
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Dimension {
    width: u32,
    height: u32,
    depth: u32,
}

pub trait DimensionOf {
    fn of(self) -> Dimension;
}

impl DimensionOf for (u32, u32) {
    fn of(self) -> Dimension {
        Dimension::of((self.0, self.1, 1))
    }
}

impl DimensionOf for (u32, u32, u32) {
    fn of(self) -> Dimension {
        if self.0 < 1 {
            panic!("Dimension width must be positive");
        }
        if self.1 < 1 {
            panic!("Dimension height must be positive");
        }
        if self.2 < 1 {
            panic!("Dimension depth must be positive");
        }

        Dimension {
            width: self.0,
            height: self.1,
            depth: self.2,
        }
    }
}

impl Dimension {
    pub fn of<A: DimensionOf>(args: A) -> Dimension {
        args.of()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn contains(&self, location: &Location) -> bool {
        let test_x: Result<u32, TryFromIntError> = location.x().try_into();
        if test_x.is_err() {
            return false;
        }

        let test_y: Result<u32, TryFromIntError> = location.y().try_into();
        if test_y.is_err() {
            return false;
        }

        let test_z: Result<u32, TryFromIntError> = location.z().try_into();
        if test_z.is_err() {
            return false;
        }

        let x = test_x.unwrap();
        let y = test_y.unwrap();
        let z = test_z.unwrap();

        x < self.width && y < self.height && z < self.depth
    }
}

/// Defines the absolute separation between two [`Location`] instances.
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Distance {
    x: i32,
    y: i32,
    z: i32,
}

static ZERO: Distance = Distance { x: 0, y: 0, z: 0 };
static MAX: Distance = Distance {
    x: i32::MAX,
    y: i32::MAX,
    z: i32::MAX,
};

pub trait DistanceOf {
    fn of(self) -> Distance;
}

impl DistanceOf for (i32, i32) {
    fn of(self) -> Distance {
        Distance::of((self.0, self.1, 0))
    }
}

impl DistanceOf for (i32, i32, i32) {
    fn of(self) -> Distance {
        Distance {
            x: self.0.abs(),
            y: self.1.abs(),
            z: self.2.abs(),
        }
    }
}

impl Distance {
    pub fn zero() -> &'static Distance {
        &ZERO
    }

    pub fn max() -> &'static Distance {
        &MAX
    }

    pub fn of<A: DistanceOf>(args: A) -> Distance {
        args.of()
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn z(&self) -> i32 {
        self.z
    }

    pub fn between(start: &Location, end: &Location) -> Distance {
        Distance::of((
            end.x() - start.x(),
            end.y() - start.y(),
            end.z() - start.z(),
        ))
    }

    pub fn is_within(&self, distance: Distance) -> bool {
        self.x <= distance.x() && self.y <= distance.y() && self.z <= distance.z()
    }
}

/// Defines a path between start and end [`Location`] instances.
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Line {
    locations: BTreeSet<Location>,
}

impl Line {
    pub fn between(start: &Location, end: &Location) -> Line {
        let mut locations: BTreeSet<Location> = BTreeSet::new();
        locations.insert(*start);

        let d = Distance::between(&start, &end);
        let n: f32 = cmp::max(d.x(), cmp::max(d.y(), d.z())) as f32;

        let sx: f32 = d.x() as f32 / n;
        let sy: f32 = d.y() as f32 / n;
        let sz: f32 = d.z() as f32 / n;

        let mut px: f32 = start.x() as f32;
        let mut py: f32 = start.y() as f32;
        let mut pz: f32 = start.z() as f32;
        for _ in 0..(n as i32) {
            px += sx;
            py += sy;
            pz += sz;

            locations.insert(Location::at((
                px.round() as i32,
                py.round() as i32,
                pz.round() as i32,
            )));
        }

        Line { locations }
    }

    pub fn start(&self) -> &Location {
        self.locations.first().unwrap()
    }

    pub fn end(&self) -> &Location {
        self.locations.last().unwrap()
    }

    pub fn contains(&self, location: &Location) -> bool {
        self.locations.contains(location)
    }
}

/// Defines a location in space without concern for what may or may not be at that location.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Location {
    x: i32,
    y: i32,
    z: i32,
}

pub trait LocationAt {
    fn at(self) -> Location;
}

impl LocationAt for (i32, i32) {
    fn at(self) -> Location {
        Location::at((self.0, self.1, 0))
    }
}

impl LocationAt for (i32, i32, i32) {
    fn at(self) -> Location {
        Location {
            x: self.0,
            y: self.1,
            z: self.2,
        }
    }
}

impl Location {
    pub fn at<A: LocationAt>(args: A) -> Location {
        args.at()
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn z(&self) -> i32 {
        self.z
    }

    pub fn go(&self, vector: &Vector) -> Location {
        Location::at((
            self.x + vector.x(),
            self.y + vector.y(),
            self.z + vector.z(),
        ))
    }

    pub fn is_within(&self, distance: &Distance, location: &Location) -> bool {
        let other_x = location.x();
        let other_y = location.y();
        let other_z = location.z();

        (self.x - other_x).abs() <= distance.x()
            && (self.y - other_y).abs() <= distance.y()
            && (self.z - other_z).abs() <= distance.z()
    }
}

/// Defines the distance and direction to go from one [`Location`] to another.
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

pub trait VectorOf {
    fn of(self) -> Vector;
}

impl VectorOf for (i32, i32) {
    fn of(self) -> Vector {
        Vector::of((self.0, self.1, 0))
    }
}

impl VectorOf for (i32, i32, i32) {
    fn of(self) -> Vector {
        Vector {
            x: self.0,
            y: self.1,
            z: self.2,
        }
    }
}

impl Vector {
    pub fn of<A: VectorOf>(args: A) -> Vector {
        args.of()
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn z(&self) -> i32 {
        self.z
    }

    pub fn from(start: &Location, end: &Location) -> Vector {
        Vector::of((
            end.x() - start.x(),
            end.y() - start.y(),
            end.z() - start.z(),
        ))
    }
}
