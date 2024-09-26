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

use multiset::HashMultiSet;
use rstest::rstest;

use crate::{
    component::{Bag, BagImpl, ErrorKind, Piece, PieceFactory, Placement, PlacementImpl},
    lang::Letter,
    space::{Location, Orientations},
};

#[rstest]
#[case(vec![], true)]
#[case(vec![None], false)]
fn bag_impl_is_empty(
    #[case] input_letters: Vec<Option<Box<dyn Letter>>>,
    #[case] empty: bool,
) {
    // given
    let letters = HashMultiSet::from_iter(input_letters.into_iter());
    let piece_factory = Box::new(TestPieceFactory {});

    // when
    let result = BagImpl::new(letters, piece_factory).is_empty();

    // then
    assert_eq!(result, empty)
}

#[rstest]
#[case(vec![], 0)]
#[case(vec![None], 1)]
#[case(vec![Some(Box::new(TestLetter::A)as Box<dyn Letter>)], 1)]
#[case(vec![None, Some(Box::new(TestLetter::A)as Box<dyn Letter>)], 2)]
#[case(vec![Some(Box::new(TestLetter::A)as Box<dyn Letter>), Some(Box::new(TestLetter::B)as Box<dyn Letter>)], 2)]
#[case(vec![Some(Box::new(TestLetter::A)as Box<dyn Letter>), Some(Box::new(TestLetter::A)as Box<dyn Letter>), Some(Box::new(TestLetter::B)as Box<dyn Letter>), Some(Box::new(TestLetter::B)as Box<dyn Letter>), Some(Box::new(TestLetter::B)as Box<dyn Letter>)], 5)]
fn bag_impl_count(
    #[case] input_letters: Vec<Option<Box<dyn Letter>>>,
    #[case] count: usize,
) {
    // given
    let letters = HashMultiSet::from_iter(input_letters.into_iter());
    let piece_factory = Box::new(TestPieceFactory {});

    // when
    let result = BagImpl::new(letters, piece_factory).count();

    // then
    assert_eq!(result, count)
}

#[test]
fn bag_impl_random_piece_empty() {
    // given
    let letters = HashMultiSet::new();
    let piece_factory = Box::new(TestPieceFactory {});

    // when
    let result = BagImpl::new(letters, piece_factory).random_piece();

    // then
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ErrorKind::NotEnoughPieces);
}

#[test]
fn bag_impl_random_piece_not_empty() {
    // given
    let letter = Some(Box::new(TestLetter::A) as Box<dyn Letter>);

    let mut letters = HashMultiSet::new();
    letters.insert(letter.clone());
    let piece_factory = Box::new(TestPieceFactory {});

    // when
    let result = BagImpl::new(letters, piece_factory).random_piece();

    // then
    assert!(result.is_ok());
    assert_eq!(result.unwrap().letter(), &letter);
}

// TODO finish unit tests

#[rstest]
#[case(
    new_piece(Option::None, 0, true),
    new_piece(Option::None, 0, true),
    true
)]
#[case(
    new_piece(Option::None, 0, true),
    new_piece(Option::None, 1, true),
    true
)]
#[case(
    new_piece(Option::None, 0, true),
    new_piece(Option::Some(TestLetter::A), 0, true),
    true
)]
#[case(
    new_piece(Option::None, 0, true),
    new_piece(Option::Some(TestLetter::A), 1, false),
    false
)]
#[case(
    new_piece(Option::Some(TestLetter::A), 1, false),
    new_piece(Option::Some(TestLetter::A), 1, false),
    true
)]
#[case(
    new_piece(Option::Some(TestLetter::A), 1, false),
    new_piece(Option::Some(TestLetter::A), 2, false),
    true
)]
#[case(
    new_piece(Option::Some(TestLetter::A), 1, false),
    new_piece(Option::None, 1, false),
    false
)]
#[case(
    new_piece(Option::Some(TestLetter::A), 1, false),
    new_piece(Option::Some(TestLetter::B), 1, false),
    false
)]
fn piece_eq(#[case] lhs: Box<dyn Piece>, #[case] rhs: Box<dyn Piece>, #[case] expected: bool) {
    // when
    let result = lhs.eq(&rhs);

    // then
    assert_eq!(result, expected);
}

#[test]
fn placement_impl_new() {
    // given
    let start_location = Location::at((0, 0, 0));
    let orientation = Orientations::x();
    let pieces: Vec<Box<dyn Piece>> = vec![new_piece(Option::Some(TestLetter::A), 1, false)];

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
#[case(PlacementImpl::new(Location::at((1, 1, 1)), Orientations::x(), vec![]), PlacementImpl::new(Location::at((1, 1, 1)), Orientations::x(), vec![new_piece(Option::Some(TestLetter::A), 1, false)]), false)]
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum TestLetter {
    A,
    B,
}
impl Letter for TestLetter {
    fn character(&self) -> char {
        self.to_string().chars().next().unwrap()
    }
}
impl Display for TestLetter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self)
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

#[derive(Debug)]
struct TestPieceFactory {}
impl PieceFactory for TestPieceFactory {
    fn create_piece(&self, letter: Option<Box<dyn Letter>>) -> Box<dyn Piece> {
        let wild = letter.is_none();
        Box::new(TestPiece {
            letter: letter.map(|l| l as Box<dyn Letter>).or(None),
            value: 1,
            wild,
        })
    }
}

fn new_piece(letter: Option<TestLetter>, value: i32, wild: bool) -> Box<TestPiece> {
    Box::new(TestPiece {
        letter: letter.map(|l| Box::new(l) as Box<dyn Letter>).or(None),
        value,
        wild,
    })
}
