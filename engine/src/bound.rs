#[derive(Debug)]
#[repr(u8)]
pub enum Bound {
    Exact = 0,
    Beta = 1,
    Alpha = 2,
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
        matches!(
            (self, other),
            (Bound::Exact, Bound::Exact)
                | (Bound::Beta, Bound::Beta)
                | (Bound::Alpha, Bound::Alpha)
        )
    }
}
