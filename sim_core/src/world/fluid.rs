#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FluidType {
    Water,
    Magma,
    Slime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fluid {
    pub kind: FluidType,
    pub depth: u8,
}
