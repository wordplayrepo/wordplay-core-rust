#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub trait Of {
    fn of(self) -> Vector;
}

impl Vector {
    pub fn of<A>(args: A) -> Vector
    where
        A: Of,
    {
        args.of()
    }
}

impl Of for (i32, i32) {
    fn of(self) -> Vector {
        return Vector {
            x: self.0,
            y: self.1,
            z: 0,
        };
    }
}

impl Of for (i32, i32, i32) {
    fn of(self) -> Vector {
        return Vector {
            x: self.0,
            y: self.1,
            z: self.2,
        };
    }
}
