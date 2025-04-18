/// Represents score bounds for chess positions in alpha-beta search.
/// Used in transposition table entries to determine the reliability
/// and type of stored position scores.
#[derive(Debug)]
#[repr(u8)]
pub enum Bound {
    /// Exact score - The stored value is precise
    Exact = 0,
    /// Beta bound - The true score is at least as good as the stored value
    Beta = 1,
    /// Alpha bound - The true score is at most as good as the stored value
    Alpha = 2,
}

/// Implements cloning functionality for Bound enum
impl Clone for Bound {
    /// Creates an exact copy of the Bound value.
    ///
    /// # Returns
    /// * A new Bound instance with the same variant
    fn clone(&self) -> Self {
        match self {
            Bound::Exact => Bound::Exact,
            Bound::Beta => Bound::Beta,
            Bound::Alpha => Bound::Alpha,
        }
    }
}

/// Implements equality comparison for Bound enum
impl PartialEq for Bound {
    /// Checks if two Bound values are equal.
    ///
    /// # Arguments
    /// * `self` - Reference to this Bound instance
    /// * `other` - Reference to Bound instance to compare against
    ///
    /// # Returns
    /// * `true` if both instances represent the same bound type
    /// * `false` otherwise
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Bound::Exact, Bound::Exact)
                | (Bound::Beta, Bound::Beta)
                | (Bound::Alpha, Bound::Alpha)
        )
    }
}
