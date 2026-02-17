#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MaterialType {
    #[default]
    None,
    Stone,
    Dirt,
    Wood,
    Sand,
}
