pub trait Merge<'de>: serde::de::Deserialize<'de> {
    // Merge two instances of the same type, other is merged into self.
    // For recursive structures, merge from the top down so the closest one is merged last.
    fn merge(&mut self, other: &Self);
}
