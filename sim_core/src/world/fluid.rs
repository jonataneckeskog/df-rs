#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FluidType {
    #[default]
    Water,
    Magma,
    Slime,
}
