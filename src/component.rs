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

use std::collections::{HashMap, HashSet};

use indexmap::IndexSet;

use crate::{
    lang::Letter,
    rust::{DynEq, DynHash, DynOrd},
    space::{Dimension, Distance, Location, Orientation},
};

/// A board represents the playing area for a game. It consists of a set of [`Tile`] on which a
/// [`Placement`] of [`Piece`] can be made. These tiles can also have other attributes that affect
/// the score or gameplay when a piece is played on them.
pub trait Board {
    /// Retrieve the sizing of this board.
    fn dimension(&self) -> Dimension;

    /// Determine whether or not the given [`Placement`] is valid on this board given the current
    /// state of other placements (if any exist).
    fn valid<T: Placement>(&self, placement: &T) -> bool;

    /// Calculate the score that the given [`Placement`] would receive.
    fn calculate_points<T: Placement>(&self, placement: &T) -> i32;

    /// Commit the given [`Placement`] to this board.
    fn place<T: Placement>(&mut self, placement: T) -> Result<i32, Error>;

    /// Retrieve the set of [`Tile`] that make up this board.
    fn tiles<T: TileSet>(&self) -> &T;

    /// Retrieve the starting [`Location`] for this board.
    fn start(&self) -> &Location;

    /// Retrieve the allowed [`Placement`] [`Orientation`] for the board.
    fn orientations(&self) -> &IndexSet<Box<dyn Orientation>>;
}

/// A piece represents a game token that is contains a [`Letter`] and has attributes such as a value
/// and a wildcard status.
pub trait Piece: Copy + DynEq + DynHash {
    /// Set the [`Letter`] that this piece represents.
    fn set_letter<T: Letter>(&mut self, letter: T);

    /// Retrieve the [`Letter`] that this piece represents.
    fn letter<T: Letter>(&self) -> &T;

    /// Retrieve the base value of this piece when used in a placement.
    fn value(&self) -> i32;

    /// Determine whether or not this piece represents a wildcard (no specific [`Letter`] until one
    /// is chosen).
    fn wild(&self) -> bool;
}

/// A placement is a specific grouping of pieces with a location and orientation.
pub trait Placement: DynEq + DynHash {
    /// Retrieve the starting location of this placement.
    fn start_location(&self) -> &Location;

    /// Retrieve the spatial orientation of this placement (e.g. along the x-axis).
    fn orientation<T: Orientation>(&self) -> &T;

    /// Retrieve the pieces contained within this placement.
    fn pieces<T: Piece>(&self) -> &Vec<T>;
}

/// A tile represents a location on the game [`Board`] that can be occupied by a [`Piece`].
pub trait Tile: DynOrd + DynHash {
    /// Retrieve this tile's location.
    fn location(&self) -> &Location;

    /// Set the [`Piece`] that occupies this tile.
    fn set_piece<T: Piece>(&mut self, piece: T);

    /// Get the piece that occupies this tile or nothing if empty.
    fn piece<T: Piece>(&self) -> Option<&T>;

    /// Retrieve the value of this tile taking into account only the value of a [`Piece`] on the
    /// tile. Attributes are not considered.
    fn base_value(&self) -> i32;

    /// Add the given attribute to this tile that may affect the score or gameplay.
    fn add_attribute<T: TileAttribute>(&mut self, attribute: T);

    /// Remove the given attribute from this tile that may affect the score or gameplay.
    fn remove_attribute<T: TileAttribute>(&mut self, attribute: T);

    /// Retrieve the set of attributes associated with this tile.
    fn attributes<T: TileAttribute>(&self) -> HashSet<T>;
}

/// A tile attribute represents a modifier that is applied to the value of a [`Piece`] placed on a
/// [`Tile`] or nearby tiles to increase or decrease the final point score or affect gameplay.
pub trait TileAttribute {
    /// Modify the given value based on the rules of this attribute.
    ///
    /// The [`Distance`] is from the [`Tile`] to which this attribute belongs to where the given
    /// value was found. For example, if this value comes from an adjacent tile, the [`Distance`]
    /// would be 1 in at least one direction and no more than 1 in any direction.
    ///
    /// `sameWord` indicates if the value was found on a tile in the same word as the tile having
    /// this attribute.
    fn modify_value(&self, value: i32, distance: &Distance, same_word: bool) -> i32;

    /// Determine whether or not this attribute should be visible to the player before a piece is
    /// placed on the tile.
    fn visible(&self) -> bool;
}

/// A tile set is a collection of [`Tile`] belonging to a [`Board`].
pub trait TileSet {
    /// Remove all [`Tile`] from this set.
    fn clear(&mut self);

    /// Retrieve the [`Tile`] at the given [`Location`]. If no such tile exists, one will be
    /// created.
    fn tile<T: Tile>(&mut self, location: &Location) -> &T;

    /// Retrieve the subset of [`Tile`] which are occupied by a [`Piece`].
    fn occupied_tiles<T: Tile>(&self) -> HashSet<&T>;

    /// Retrieve all [`TileAttribute`] for the given set of [`Location`].
    fn attributes<T: TileAttribute>(
        &self,
        locations: &HashSet<Location>,
    ) -> HashMap<Location, Vec<T>>;
}

pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

pub enum ErrorKind {
    InvalidPlacement,
}
