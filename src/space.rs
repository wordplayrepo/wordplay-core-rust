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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
    pub z: i32,
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
