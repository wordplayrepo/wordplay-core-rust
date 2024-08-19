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
#[cfg(test)]
mod vector_tests {
    use crate::space::vector::Vector;

    #[test]
    fn of_xy() {
        // given
        let x = 1;
        let y = 2;

        // when
        let result = Vector::of((x, y));

        // then
        assert_eq!(result, Vector { x, y, z: 0 });
    }

    #[test]
    fn of_xyz() {
        // given
        let x = 1;
        let y = 2;
        let z = 3;

        // when
        let result = Vector::of((x, y, z));

        // then
        assert_eq!(result, Vector { x, y, z });
    }
}
