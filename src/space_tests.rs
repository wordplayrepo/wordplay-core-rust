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
use crate::space::Dimension;
use crate::space::Distance;
use crate::space::Location;
use crate::space::Vector;
use rstest::rstest;
use std::cmp::Ordering;

// Dimension start =====

#[test]
fn dimension_of_width_height() {
    // given
    let width = 1;
    let height = 2;

    // when
    let result = Dimension::of((width, height));

    // then
    assert_eq!(result.width(), width);
    assert_eq!(result.height(), height);
    assert_eq!(result.depth(), 1);
}

// TODO finish unit tests for Dimension

// Dimension end =====

// Distance start =====

#[test]
fn distance_zero() {
    // when
    let result = Distance::zero();

    // then
    assert_eq!(result.x(), 0);
    assert_eq!(result.y(), 0);
    assert_eq!(result.z(), 0);
}

#[test]
fn distance_max() {
    // when
    let result = Distance::max();

    // then
    assert_eq!(result.x(), i32::MAX);
    assert_eq!(result.y(), i32::MAX);
    assert_eq!(result.z(), i32::MAX);
}

#[rstest]
#[case(1, 1, 1, 2, 2, 2, 1, 1, 1)]
#[case(2, 2, 2, 1, 1, 1, 1, 1, 1)]
#[case(0, 0, 0, 1, 0, 0, 1, 0, 0)]
#[case(0, 0, 0, 0, 1, 0, 0, 1, 0)]
#[case(0, 0, 0, 0, 0, 1, 0, 0, 1)]
fn distance_between(
    #[case] start_x: i32,
    #[case] start_y: i32,
    #[case] start_z: i32,
    #[case] end_x: i32,
    #[case] end_y: i32,
    #[case] end_z: i32,
    #[case] distance_x: i32,
    #[case] distance_y: i32,
    #[case] distance_z: i32,
) {
    // given
    let start = Location::at((start_x, start_y, start_z));
    let end = Location::at((end_x, end_y, end_z));

    // when
    let result = Distance::between(&start, &end);

    // then
    assert_eq!(result, Distance::of((distance_x, distance_y, distance_z)));
}

#[test]
fn distance_of_xy() {
    // given
    let x = 1;
    let y = 2;

    // when
    let result = Distance::of((x, y));

    // then
    assert_eq!(result.x(), x);
    assert_eq!(result.y(), y);
    assert_eq!(result.z(), 0);
}

#[test]
fn distance_of_xyz() {
    // given
    let x = 1;
    let y = 2;
    let z = 3;

    // when
    let result = Distance::of((x, y, z));

    // then
    assert_eq!(result.x(), x);
    assert_eq!(result.y(), y);
    assert_eq!(result.z(), z);
}

#[rstest]
#[case(-1,0,0)]
#[case(0,-1,0)]
#[case(0,0,-1)]
fn distance_of_xyz_negative(#[case] x: i32, #[case] y: i32, #[case] z: i32) {
    // when
    let result = Distance::of((x, y, z));

    // then
    assert_eq!(result.x(), x.abs());
    assert_eq!(result.y(), y.abs());
    assert_eq!(result.z(), z.abs());
}

#[rstest]
#[case(0, 0, 0, true)]
#[case(1, 0, 0, true)]
#[case(0, 1, 0, true)]
#[case(0, 0, 1, true)]
#[case(0, 1, 1, true)]
#[case(1, 0, 1, true)]
#[case(1, 1, 0, true)]
#[case(1, 1, 1, true)]
#[case(2, 0, 0, false)]
#[case(0, 2, 0, false)]
#[case(0, 0, 2, false)]
fn distance_is_within(#[case] x: i32, #[case] y: i32, #[case] z: i32, #[case] expected: bool) {
    // given
    let distance = Distance::of((x, y, z));

    // when
    let result = distance.is_within(Distance::of((1, 1, 1)));

    // then
    assert_eq!(result, expected)
}

#[rstest]
#[case(1, 1, 1, 1, 1, 1, Ordering::Equal)]
#[case(1, 1, 1, 2, 1, 1, Ordering::Less)]
#[case(1, 1, 1, 1, 2, 1, Ordering::Less)]
#[case(1, 1, 1, 1, 1, 2, Ordering::Less)]
#[case(2, 2, 2, 3, 1, 1, Ordering::Less)]
fn distance_cmp(
    #[case] lhs_x: i32,
    #[case] lhs_y: i32,
    #[case] lhs_z: i32,
    #[case] rhs_x: i32,
    #[case] rhs_y: i32,
    #[case] rhs_z: i32,
    #[case] expected: Ordering,
) {
    // given
    let lhs = Distance::of((lhs_x, lhs_y, lhs_z));
    let rhs = Distance::of((rhs_x, rhs_y, rhs_z));

    // when
    let result = lhs.cmp(&rhs);

    // then
    assert_eq!(result, expected);
}

// Distance end =====

// Location start =====

#[test]
fn location_at_xy() {
    // given
    let x = 1;
    let y = 2;

    // when
    let result = Location::at((x, y));

    // then
    assert_eq!(result.x(), x);
    assert_eq!(result.y(), y);
    assert_eq!(result.z(), 0);
}

#[test]
fn location_at_xyz() {
    // given
    let x = 1;
    let y = 2;
    let z = 3;

    // when
    let result = Location::at((x, y, z));

    // then
    assert_eq!(result.x(), x);
    assert_eq!(result.y(), y);
    assert_eq!(result.z(), z);
}

#[test]
fn location_go_positive() {
    // given
    let location = Location::at((1, 1, 1));
    let vector = Vector::of((1, 1, 1));

    // when
    let result = location.go(&vector);

    // then
    assert_eq!(result, Location::at((2, 2, 2)));
}

#[test]
fn location_go_negative() {
    // given
    let location = Location::at((2, 2, 2));
    let vector = Vector::of((-1, -1, -1));

    // when
    let result = location.go(&vector);

    // then
    assert_eq!(result, Location::at((1, 1, 1)));
}

#[rstest]
#[case(1, 1, 1, 3, 3, 3, true)]
#[case(2, 1, 1, 4, 3, 3, true)]
#[case(1, 2, 1, 3, 4, 3, true)]
#[case(1, 1, 2, 3, 3, 4, true)]
#[case(1, 1, 1, 1, 1, 1, true)]
#[case(1, 1, 1, 4, 4, 4, false)]
#[case(1, 1, 1, 4, 3, 3, false)]
#[case(1, 1, 1, 3, 4, 3, false)]
#[case(1, 1, 1, 3, 3, 4, false)]
fn location_is_within(
    #[case] distance_x: i32,
    #[case] distance_y: i32,
    #[case] distance_z: i32,
    #[case] target_x: i32,
    #[case] target_y: i32,
    #[case] target_z: i32,
    #[case] expected: bool,
) {
    // given
    let start = Location::at((2, 2, 2));
    let distance = Distance::of((distance_x, distance_y, distance_z));
    let target = Location::at((target_x, target_y, target_z));

    // when
    let result = start.is_within(&distance, &target);

    // then
    assert_eq!(result, expected);
}

#[rstest]
#[case(1, 1, 1, 1, 1, 1, Ordering::Equal)]
#[case(1, 1, 1, 2, 1, 1, Ordering::Less)]
#[case(1, 1, 1, 1, 2, 1, Ordering::Less)]
#[case(1, 1, 1, 1, 1, 2, Ordering::Less)]
#[case(2, 2, 2, 3, 1, 1, Ordering::Less)]
fn location_cmp(
    #[case] lhs_x: i32,
    #[case] lhs_y: i32,
    #[case] lhs_z: i32,
    #[case] rhs_x: i32,
    #[case] rhs_y: i32,
    #[case] rhs_z: i32,
    #[case] expected: Ordering,
) {
    // given
    let lhs = Location::at((lhs_x, lhs_y, lhs_z));
    let rhs = Location::at((rhs_x, rhs_y, rhs_z));

    // when
    let result = lhs.cmp(&rhs);

    // then
    assert_eq!(result, expected);
}

// Location end =====

// Vector start =====

#[test]
fn vector_of_xy() {
    // given
    let x = 1;
    let y = 2;

    // when
    let result = Vector::of((x, y));

    // then
    assert_eq!(result.x(), x);
    assert_eq!(result.y(), y);
    assert_eq!(result.z(), 0);
}

#[test]
fn vector_of_xyz() {
    // given
    let x = 1;
    let y = 2;
    let z = 3;

    // when
    let result = Vector::of((x, y, z));

    // then
    assert_eq!(result.x(), x);
    assert_eq!(result.y(), y);
    assert_eq!(result.z(), z);
}

#[rstest]
#[case(1, 1, 1, 2, 1, 1)]
#[case(1, 1, 1, 1, 2, 1)]
#[case(1, 1, 1, 1, 1, 2)]
#[case(2, 1, 1, 1, 1, 1)]
#[case(1, 2, 1, 1, 1, 1)]
#[case(1, 1, 2, 1, 1, 1)]
#[case(-1,-1,-1,1,1,1)]
fn vector_from(
    #[case] start_x: i32,
    #[case] start_y: i32,
    #[case] start_z: i32,
    #[case] end_x: i32,
    #[case] end_y: i32,
    #[case] end_z: i32,
) {
    // given
    let start = Location::at((start_x, start_y, start_z));
    let end = Location::at((end_x, end_y, end_z));

    // when
    let result = Vector::from(&start, &end);

    // then
    assert_eq!(
        result,
        Vector::of((end_x - start_x, end_y - start_y, end_z - start_z))
    );
}

#[rstest]
#[case(1, 1, 1, 1, 1, 1, Ordering::Equal)]
#[case(1, 1, 1, 2, 1, 1, Ordering::Less)]
#[case(1, 1, 1, 1, 2, 1, Ordering::Less)]
#[case(1, 1, 1, 1, 1, 2, Ordering::Less)]
#[case(2, 2, 2, 3, 1, 1, Ordering::Less)]
fn vector_cmp(
    #[case] lhs_x: i32,
    #[case] lhs_y: i32,
    #[case] lhs_z: i32,
    #[case] rhs_x: i32,
    #[case] rhs_y: i32,
    #[case] rhs_z: i32,
    #[case] expected: Ordering,
) {
    // given
    let lhs = Vector::of((lhs_x, lhs_y, lhs_z));
    let rhs = Vector::of((rhs_x, rhs_y, rhs_z));

    // when
    let result = lhs.cmp(&rhs);

    // then
    assert_eq!(result, expected);
}

// Vector end =====
