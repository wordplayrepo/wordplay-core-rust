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
/// Defines the absolute separation between two [`Location`] instances.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
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

pub trait OfDistance {
    fn of(self) -> Distance;
}

impl Distance {
    pub fn zero() -> &'static Distance {
        return &ZERO;
    }

    pub fn max() -> &'static Distance {
        return &MAX;
    }

    pub fn of<A>(args: A) -> Distance
    where
        A: OfDistance,
    {
        args.of()
    }

    pub fn x(&self) -> i32 {
        return self.x;
    }

    pub fn y(&self) -> i32 {
        return self.y;
    }

    pub fn z(&self) -> i32 {
        return self.z;
    }

    pub fn between(start: &Location, end: &Location) -> Distance {
        return Distance::of((
            end.x() - start.x(),
            end.y() - start.y(),
            end.z() - start.z(),
        ));
    }

    pub fn is_within(&self, distance: Distance) -> bool {
        return self.x <= distance.x() && self.y <= distance.y() && self.z <= distance.z();
    }
}

impl OfDistance for (i32, i32) {
    fn of(self) -> Distance {
        return Distance {
            x: self.0.abs(),
            y: self.1.abs(),
            z: 0,
        };
    }
}

impl OfDistance for (i32, i32, i32) {
    fn of(self) -> Distance {
        return Distance {
            x: self.0.abs(),
            y: self.1.abs(),
            z: self.2.abs(),
        };
    }
}

/// Defines a location in space without concern for what may or may not be at that location.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Location {
    x: i32,
    y: i32,
    z: i32,
}

pub trait AtLocation {
    fn at(self) -> Location;
}

impl Location {
    pub fn at<A>(args: A) -> Location
    where
        A: AtLocation,
    {
        args.at()
    }

    pub fn x(&self) -> i32 {
        return self.x;
    }

    pub fn y(&self) -> i32 {
        return self.y;
    }

    pub fn z(&self) -> i32 {
        return self.z;
    }

    pub fn go(&self, vector: &Vector) -> Location {
        return Location::at((
            self.x + vector.x(),
            self.y + vector.y(),
            self.z + vector.z(),
        ));
    }
}

impl AtLocation for (i32, i32) {
    fn at(self) -> Location {
        return Location {
            x: self.0,
            y: self.1,
            z: 0,
        };
    }
}

impl AtLocation for (i32, i32, i32) {
    fn at(self) -> Location {
        return Location {
            x: self.0,
            y: self.1,
            z: self.2,
        };
    }
}

/// Defines the distance and direction to go from one [`Location`] to another.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

pub trait OfVector {
    fn of(self) -> Vector;
}

impl Vector {
    pub fn of<A>(args: A) -> Vector
    where
        A: OfVector,
    {
        args.of()
    }

    pub fn x(&self) -> i32 {
        return self.x;
    }

    pub fn y(&self) -> i32 {
        return self.y;
    }

    pub fn z(&self) -> i32 {
        return self.z;
    }

    pub fn from(start: &Location, end: &Location) -> Vector {
        return Vector::of((
            end.x() - start.x(),
            end.y() - start.y(),
            end.z() - start.z(),
        ));
    }
}

impl OfVector for (i32, i32) {
    fn of(self) -> Vector {
        return Vector {
            x: self.0,
            y: self.1,
            z: 0,
        };
    }
}

impl OfVector for (i32, i32, i32) {
    fn of(self) -> Vector {
        return Vector {
            x: self.0,
            y: self.1,
            z: self.2,
        };
    }
}
