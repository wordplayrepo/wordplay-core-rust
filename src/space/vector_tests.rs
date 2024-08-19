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
        assert_eq!(result, Vector { x: 1, y: 2, z: 0 });
    }
}
