#![no_std]
#![doc = include_str!("../README.md")]

use core::{
    ops::Deref,
    pin::Pin,
};

/// Consume self into reference
pub trait IntoRef<'a>: 'a {
    type Out: ?Sized;

    /// Consume self into reference
    fn into_ref(self) -> &'a Self::Out;
}

/// Consume self into mutable reference
pub trait IntoRefMut<'a>: IntoRef<'a> {
    /// Consume self into mutable reference
    fn into_ref_mut(self) -> &'a mut Self::Out;
}
impl<'a, T: ?Sized> IntoRef<'a> for &'a T {
    type Out = T;

    fn into_ref(self) -> &'a Self::Out {
        self
    }
}
impl<'a, T: ?Sized> IntoRef<'a> for &'a mut T {
    type Out = T;

    fn into_ref(self) -> &'a Self::Out {
        self
    }
}
impl<'a, T: ?Sized> IntoRefMut<'a> for &'a mut T {
    fn into_ref_mut(self) -> &'a mut Self::Out {
        self
    }
}
impl<'a, P> IntoRef<'a> for Pin<P>
where P: Deref + IntoRef<'a>,
      P::Target: Unpin,
{
    type Out = P::Out;

    fn into_ref(self) -> &'a Self::Out {
        Pin::into_inner(self).into_ref()
    }
}
impl<'a, P> IntoRefMut<'a> for Pin<P>
where P: Deref + IntoRefMut<'a>,
      P::Target: Unpin,
{
    fn into_ref_mut(self) -> &'a mut Self::Out {
        Pin::into_inner(self).into_ref_mut()
    }
}
