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

use std::fmt::{Display, Formatter, Result};

use rstest::rstest;

use crate::{
    component::{Piece, Placement, PlacementImpl},
    lang::Letter,
    space::{Location, Orientations},
};

#[test]
fn placement_impl_new() {
    // given
    let start_location = Location::at((0, 0, 0));
    let orientation = Orientations::x();
    let pieces: Vec<Box<dyn Piece>> = vec![new_piece(Option::Some('A'), 1, false)];

    // when
    let result = PlacementImpl::new(start_location, orientation.clone(), pieces.clone());

    // then
    assert_eq!(result.start_location(), &start_location);
    assert_eq!(result.orientation(), &*orientation);
    assert_eq!(result.pieces(), &pieces);
}

#[rstest]
#[case(PlacementImpl::new(Location::at((1, 1, 1)), Orientations::x(), vec![]), PlacementImpl::new(Location::at((1, 1, 1)), Orientations::x(), vec![]), true)]
#[case(PlacementImpl::new(Location::at((1, 1, 1)), Orientations::x(), vec![]), PlacementImpl::new(Location::at((2, 2, 2)), Orientations::x(), vec![]), false)]
#[case(PlacementImpl::new(Location::at((1, 1, 1)), Orientations::x(), vec![]), PlacementImpl::new(Location::at((1, 1, 1)), Orientations::y(), vec![]), false)]
#[case(PlacementImpl::new(Location::at((1, 1, 1)), Orientations::x(), vec![]), PlacementImpl::new(Location::at((1, 1, 1)), Orientations::x(), vec![new_piece(Option::Some('A'), 1, false)]), false)]
fn placement_impl_eq(
    #[case] lhs: PlacementImpl,
    #[case] rhs: PlacementImpl,
    #[case] expected: bool,
) {
    // when
    let result = lhs.eq(&rhs);

    // then
    assert_eq!(result, expected);
}

// TODO test Piece equality

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct TestLetter {
    character: char,
}
impl Letter for TestLetter {
    fn character(&self) -> char {
        self.character
    }
}
impl Display for TestLetter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.character)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct TestPiece {
    letter: Option<Box<dyn Letter>>,
    value: i32,
    wild: bool,
}
impl Piece for TestPiece {
    fn set_letter(&mut self, letter: Option<Box<dyn Letter>>) {
        self.letter = letter;
    }

    fn letter(&self) -> &Option<Box<dyn Letter>> {
        &self.letter
    }

    fn value(&self) -> i32 {
        self.value
    }

    fn wild(&self) -> bool {
        self.wild
    }
}

fn new_piece(character: Option<char>, value: i32, wild: bool) -> Box<TestPiece> {
    Box::new(TestPiece {
        letter: character.map_or_else(
            || Option::None,
            |c| Option::Some(Box::new(TestLetter { character: c }) as Box<dyn Letter>),
        ),
        value,
        wild,
    })
}
