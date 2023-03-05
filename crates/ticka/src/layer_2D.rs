


// 2 dimensional data
// dense filled,
// like 
// - Temperature
// - Lava Amount
// - Water Amount
// great to get processed by shaders.
//
// But also data like Movement Cost Calculation,
// that can be cached for different character types.



pub type OptionalLayer2D<TData> = Layer2D<Option<TData>>;


pub struct Rect {
    pub width: u32,
    pub height: u32
}

pub struct Layer2D<TData> {
    pub data: Vec<Vec<TData>>, // <- this is pobably not the perfect solution for passing to shaders.
    pub rect: Rect
}