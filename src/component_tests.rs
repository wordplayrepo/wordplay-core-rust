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

use rstest::rstest;

use crate::{
    component::{Piece, Placement, PlacementImpl},
    space::{Location, Orientations},
};

#[test]
fn placement_impl_new() {
    // given
    let start_location = Location::at((0, 0, 0));
    let orientation = Orientations::x();
    let pieces: Vec<Box<dyn Piece>> = vec![];

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
// TODO add case for changing pieces
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
