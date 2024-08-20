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
/// Defines a location in space without concern for what may or may not be at that location.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Location {
    x: i32,
    y: i32,
    z: i32,
}

pub trait At {
    fn at(self) -> Location;
}

impl Location {
    pub fn at<A>(args: A) -> Location
    where
        A: At,
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
        return Location::at((self.x + vector.x(), self.y + vector.y(), self.z + vector.z()))
    }
}

impl At for (i32, i32) {
    fn at(self) -> Location {
        return Location {
            x: self.0,
            y: self.1,
            z: 0,
        };
    }
}

impl At for (i32, i32, i32) {
    fn at(self) -> Location {
        return Location {
            x: self.0,
            y: self.1,
            z: self.2,
        };
    }
}

/// Defines the separation between two [`Location`] objects.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

pub trait Of {
    fn of(self) -> Vector;
}

impl Vector {
    pub fn of<A>(args: A) -> Vector
    where
        A: Of,
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
        return Vector::of((end.x() - start.x(), end.y() - start.y(), end.z() - start.z()));
    }
}

impl Of for (i32, i32) {
    fn of(self) -> Vector {
        return Vector {
            x: self.0,
            y: self.1,
            z: 0,
        };
    }
}

impl Of for (i32, i32, i32) {
    fn of(self) -> Vector {
        return Vector {
            x: self.0,
            y: self.1,
            z: self.2,
        };
    }
}
