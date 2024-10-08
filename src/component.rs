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

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::{Hash, Hasher},
};

use dyn_clone::{clone_trait_object, DynClone};
use indexmap::IndexSet;

use crate::{
    lang::Letter,
    rust::{DynEq, DynHash, DynOrd},
    space::{Dimension, Distance, Location, Orientation},
};

/// A board represents the playing area for a game. It consists of a set of [`Tile`] on which a
/// [`Placement`] of [`Piece`] can be made. These tiles can also have other attributes that affect
/// the score or gameplay when a piece is played on them.
pub trait Board: Debug + DynClone {
    /// Retrieve the sizing of this board.
    fn dimension(&self) -> Dimension;

    /// Determine whether or not the given [`Placement`] is valid on this board given the current
    /// state of other placements (if any exist).
    fn valid(&self, placement: &dyn Placement) -> bool;

    /// Calculate the score that the given [`Placement`] would receive.
    fn calculate_points(&self, placement: &dyn Placement) -> i32;

    /// Commit the given [`Placement`] to this board.
    fn place(&mut self, placement: dyn Placement) -> Result<i32, Error>;

    /// Retrieve the set of [`Tile`] that make up this board.
    fn tiles(&self) -> &dyn TileSet;

    /// Retrieve the starting [`Location`] for this board.
    fn start(&self) -> &Location;

    /// Retrieve the allowed [`Placement`] [`Orientation`] for the board.
    fn orientations(&self) -> &IndexSet<Box<dyn Orientation>>;
}

clone_trait_object!(Board);

/// A piece represents a game token that contains a [`Letter`] and has attributes such as a value
/// and a wildcard status.
pub trait Piece: Debug + DynClone + DynEq + DynHash {
    /// Set the [`Letter`] that this piece represents.
    fn set_letter(&mut self, letter: Option<Box<dyn Letter>>);

    /// Retrieve the [`Letter`] that this piece represents.
    fn letter(&self) -> &Option<Box<dyn Letter>>;

    /// Retrieve the base value of this piece when used in a placement.
    fn value(&self) -> i32;

    /// Determine whether or not this piece represents a wildcard (no specific [`Letter`] until one
    /// is chosen).
    fn wild(&self) -> bool;
}

clone_trait_object!(Piece);

impl Eq for dyn Piece {}

impl Hash for dyn Piece {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state)
    }
}

impl PartialEq<dyn Piece> for dyn Piece {
    fn eq(&self, other: &dyn Piece) -> bool {
        if self.wild() != other.wild() {
            return false;
        }

        /*
         * If both pieces are wild, they are the same. The letter should not be compared
         * in this case.
         */
        if self.wild() {
            return true;
        }

        if self.letter().is_none() {
            if !other.letter().is_none() {
                return false;
            }
        } else if self.letter().as_ref() != other.letter().as_ref() {
            return false;
        }

        return true;
    }
}

/// A placement is a specific grouping of pieces with a location and orientation.
pub trait Placement: Debug + DynClone + DynEq + DynHash {
    /// Retrieve the starting location of this placement.
    fn start_location(&self) -> &Location;

    /// Retrieve the spatial orientation of this placement (e.g. along the x-axis).
    fn orientation(&self) -> &dyn Orientation;

    /// Retrieve the pieces contained within this placement.
    fn pieces(&self) -> &Vec<Box<dyn Piece>>;
}

clone_trait_object!(Placement);

impl Eq for dyn Placement {}

impl Hash for dyn Placement {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state)
    }
}

impl PartialEq<dyn Placement> for dyn Placement {
    fn eq(&self, other: &dyn Placement) -> bool {
        self.as_dyn_eq() == other.as_dyn_eq()
    }
}

/// A tile represents a location on the game [`Board`] that can be occupied by a [`Piece`].
pub trait Tile: Debug + DynClone + DynEq + DynOrd + DynHash {
    /// Retrieve this tile's location.
    fn location(&self) -> &Location;

    /// Set the [`Piece`] that occupies this tile.
    fn set_piece(&mut self, piece: dyn Piece);

    /// Get the piece that occupies this tile or nothing if empty.
    fn piece(&self) -> Option<&dyn Piece>;

    /// Retrieve the value of this tile taking into account only the value of a [`Piece`] on the
    /// tile. Attributes are not considered.
    fn base_value(&self) -> i32;

    /// Add the given attribute to this tile that may affect the score or gameplay.
    fn add_attribute(&mut self, attribute: dyn TileAttribute);

    /// Remove the given attribute from this tile that may affect the score or gameplay.
    fn remove_attribute(&mut self, attribute: &dyn TileAttribute);

    /// Retrieve the set of attributes associated with this tile.
    fn attributes(&self) -> &HashSet<Box<dyn TileAttribute>>;
}

clone_trait_object!(Tile);

impl Eq for dyn Tile {}

impl Hash for dyn Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state)
    }
}

impl PartialEq<dyn Tile> for dyn Tile {
    fn eq(&self, other: &dyn Tile) -> bool {
        self.as_dyn_eq() == other.as_dyn_eq()
    }
}

impl PartialOrd<dyn Tile> for dyn Tile {
    fn partial_cmp(&self, other: &dyn Tile) -> Option<std::cmp::Ordering> {
        self.as_dyn_ord().partial_cmp(other.as_dyn_ord())
    }
}

/// A tile attribute represents a modifier that is applied to the value of a [`Piece`] placed on a
/// [`Tile`] or nearby tiles to increase or decrease the final point score or affect gameplay.
pub trait TileAttribute: Debug + DynClone {
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

clone_trait_object!(TileAttribute);

/// A tile set is a collection of [`Tile`] belonging to a [`Board`].
pub trait TileSet: Debug + DynClone {
    /// Remove all [`Tile`] from this set.
    fn clear(&mut self);

    /// Retrieve the [`Tile`] at the given [`Location`]. If no such tile exists, one will be
    /// created.
    fn tile(&mut self, location: &Location) -> &dyn Tile;

    /// Retrieve the subset of [`Tile`] which are occupied by a [`Piece`].
    fn occupied_tiles(&self) -> &HashSet<Box<dyn Tile>>;

    /// Retrieve all [`TileAttribute`] for the given set of [`Location`].
    fn attributes(
        &self,
        locations: &HashSet<Location>,
    ) -> &HashMap<Location, Vec<Box<dyn TileAttribute>>>;
}

clone_trait_object!(TileSet);

pub enum ErrorKind {
    InvalidPlacement,
}

pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

#[derive(Clone, Debug, Eq, Hash)]
pub struct PlacementImpl {
    start_location: Location,
    orientation: Box<dyn Orientation>,
    pieces: Vec<Box<dyn Piece>>,
}

impl PlacementImpl {
    pub fn new(
        start_location: Location,
        orientation: Box<dyn Orientation>,
        pieces: Vec<Box<dyn Piece>>,
    ) -> PlacementImpl {
        PlacementImpl {
            start_location,
            orientation,
            pieces,
        }
    }
}

impl PartialEq for PlacementImpl {
    fn eq(&self, other: &Self) -> bool {
        (&self.start_location).eq(&other.start_location)
            && (&self.orientation).eq(&other.orientation)
            && (&self.pieces).eq(&other.pieces)
    }
}

impl Placement for PlacementImpl {
    fn start_location(&self) -> &Location {
        &self.start_location
    }

    fn orientation(&self) -> &dyn Orientation {
        &*self.orientation
    }

    fn pieces(&self) -> &Vec<Box<dyn Piece>> {
        &self.pieces
    }
}
