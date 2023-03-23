


#[derive(Ord, Eq, PartialOrd, PartialEq, Debug, Clone, Copy)]
pub struct IndexedField2DLocation {
    x: u32,
    y: u32,
}

impl IndexedField2DLocation {
    pub fn new(x: u32, y: u32) -> Self {
        IndexedField2DLocation { x, y }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}

