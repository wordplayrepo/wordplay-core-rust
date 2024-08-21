use crate::space::Distance;
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
use crate::space::Location;
use crate::space::Vector;
use rstest::rstest;

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

// TODO finish tests for Distance

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

// Vector end =====
