#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Coordinate {
    pub fn zero() -> Coordinate {
        Coordinate { x: 0, y: 0, z: 0 }
    }
    pub fn add(&self, i: isize, j: isize, k: isize, size: usize) -> Option<Coordinate> {
        let new_x = self.x.checked_add_signed(i);
        let new_y = self.y.checked_add_signed(j);
        let new_z = self.z.checked_add_signed(k);

        if new_x.is_none() || new_y.is_none() || new_z.is_none()
            || new_x.unwrap() >= size || new_y.unwrap() >= size || new_z.unwrap() >= size 
        {
            None
        }
        else {
            Some(Coordinate { x: new_x.unwrap(), y: new_y.unwrap(), z: new_z.unwrap() })
        }
    }
}
