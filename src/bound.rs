pub enum Bound {
    Exact,
    Beta,
    Alpha,
}

impl Clone for Bound {
    fn clone(&self) -> Self {
        match self {
            Bound::Exact => Bound::Exact,
            Bound::Beta => Bound::Beta,
            Bound::Alpha => Bound::Alpha,
        }
    }
}

impl PartialEq for Bound {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Bound::Exact, Bound::Exact) => true,
            (Bound::Beta, Bound::Beta) => true,
            (Bound::Alpha, Bound::Alpha) => true,
            _ => false,
        }
    }
}
