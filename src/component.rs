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

use crate::{lang::Letter, rust::{DynEq, DynHash}, space::{Location, Orientation}};

/// A piece represents a game token that is contains a [`Letter`] and has attributes such as a value and a wildcard status.
pub trait Piece: Copy + DynEq + DynHash {
    /// Set the [`Letter`] that this piece represents.
    fn set_letter<T: Letter>(&self, letter: T);

    /// Retrieve the [`Letter`] that this piece represents.
    fn letter<T: Letter>(&self) -> T;

    /// Retrieve the base value of this piece when used in a placement.
    fn value(&self) -> i32;

    /// Determine whether or not this piece represents a wildcard (no specific [`Letter`] until one is chosen).
    fn wild(&self) -> bool;
}

/// A placement is a specific grouping of pieces with a location and orientation.
pub trait Placement: DynEq + DynHash {
    /// Retrieve the starting location of this placement.
    fn start_location(&self) -> Location;

    /// Retrieve the spatial orientation of this placement (e.g. along the x-axis).
    fn orientation<T: Orientation>(&self) -> T;

    /// Retrieve the pieces contained within this placement.
    fn pieces<T: Piece>(&self) -> Vec<T>;
}
