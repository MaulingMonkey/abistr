use crate::*;
use crate::unit::private::{Unit as _};



#[cfg(feature = "alloc")] #[doc(hidden)] pub struct EString0<E: Encoding>(alloc::vec::Vec<E::Unit>);

#[cfg(feature = "alloc")] impl<E: Encoding> EString0<E> {
    /// ### Safety
    /// *   You promise `str` is valid for [`Encoding`].
    pub(crate) unsafe fn from_vec_no_nul(mut str: alloc::vec::Vec<E::Unit>) -> Result<Self, InteriorNulError> {
        InteriorNulError::check(&str)?;
        str.push(E::Unit::NUL);
        Ok(Self(str))
    }

    /// ### Safety
    /// *   You promise `str` is valid for [`Encoding`].
    pub(crate) unsafe fn from_iter(str: impl Iterator<Item = E::Unit>) -> Result<Self, InteriorNulError> {
        let mut v = alloc::vec::Vec::new();
        v.reserve(str.size_hint().0 + 1); // +1: '\0'
        v.extend(str);
        InteriorNulError::check(&v)?;
        v.extend(Some(E::Unit::NUL));
        Ok(Self(v))
    }

    pub(crate) fn as_ptr(&self) -> *const E::Unit { self.0.as_ptr() }
}

#[cfg(feature = "alloc")] const _ : () = {
    unsafe impl<E: Encoding> AsCStr<E>      for EString0<E> { fn as_cstr        (&self) -> *const E::Unit { self.as_ptr() } }
    unsafe impl<E: Encoding> AsOptCStr<E>   for EString0<E> { fn as_opt_cstr    (&self) -> *const E::Unit { self.as_ptr() } }
};
