#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MaterialType {
    #[default]
    Stone,
    Dirt,
    Wood,
    Sand,
}
