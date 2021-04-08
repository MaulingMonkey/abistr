use crate::*;



/// <code>\[[u8]/[u16]/[u32]; N\]</code>, an array of [Unit]s.
pub trait Array : private::Array {}
impl<const N: usize> Array for [u8;  N] {}
impl<const N: usize> Array for [u16; N] {}
impl<const N: usize> Array for [u32; N] {}



pub(crate) mod private {
    use super::*;

    pub trait Array : Sized {
        type Unit : Unit;
        fn as_slice(&self) -> &[Self::Unit];
        fn as_slice_mut(&mut self) -> &mut [Self::Unit];
        fn zeroed() -> Self;
    }

    impl<U: Unit, const N: usize> Array for [U; N] {
        type Unit = U;

        fn as_slice(&self) -> &[Self::Unit] { self.as_ref() }
        fn as_slice_mut(&mut self) -> &mut [Self::Unit] { self.as_mut() }

        fn zeroed() -> Self {
            // Per private::Unit's docs, Unit must be zeroable, so an array of them should be zeroable, so this should be safe.
            // If `[T: Default; N]` ever implements `Default`, prefer it (1.51.0 only implements it for N < 32 or similar.)
            unsafe { std::mem::zeroed() }
        }
    }
}
