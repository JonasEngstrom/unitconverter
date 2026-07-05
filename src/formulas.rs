/// Stores formulas for conversion between units. For internal use.
pub(crate) enum Formula {
    Multiply { scale: f64 },
    Divide { scale: f64 },
    Affine { scale: f64, offset: f64 },
}

impl Formula {
    /// Calculate result of a formula given a Formula enum and a variable.
    pub(crate) fn apply(&self, value_to_convert: f64) -> f64 {
        match self {
            Formula::Multiply{ scale } => value_to_convert * scale,
            Formula::Divide{ scale } => value_to_convert / scale,
            Formula::Affine{ scale, offset } => scale * value_to_convert + offset,
        }
    }
}