use cioyu_rng::Rng;

/// Generic distribution trait.
///
/// Produces values of type `T`.
pub trait Distribution<T> {
    fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) -> T;
}