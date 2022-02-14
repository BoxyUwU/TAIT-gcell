#![feature(type_alias_impl_trait)]

use core::{cell::UnsafeCell, marker::PhantomData};

/// `Cell` type that can be accessed by providing a reference to the
/// [`GToken`] with the same `ID` as the `GCell`
pub struct GCell<T, ID>(UnsafeCell<T>, PhantomData<*mut ID>);
pub struct GToken<ID>(PhantomData<*mut ID>);

impl<T, ID> GCell<T, ID> {
    /// Use [`new`] to create one safely
    ///
    /// # Safety:
    /// - `ID` must not be used for any other `GCell/GToken` pairing
    pub unsafe fn manual_new(val: T, _: ID) -> (GCell<T, ID>, GToken<ID>) {
        (
            GCell(UnsafeCell::new(val), PhantomData),
            GToken(PhantomData),
        )
    }

    pub fn get_mut<'a>(&'a self, _: &'a mut GToken<ID>) -> &'a mut T {
        unsafe { &mut *self.0.get() }
    }

    pub fn get_ref<'a>(&'a self, _: &'a GToken<ID>) -> &'a T {
        unsafe { &*self.0.get() }
    }
}

#[macro_export]
/// Safe constructor for a `GCell/GToken` pair
/// ```rust
/// let (cell, token): (GCell<u32, _>, GToken<_>) = crate::new!(10);
/// ```
macro_rules! new {
    ($e:expr) => {{
        let e = $e;
        {
            pub mod ctor {
                pub type Opaque = impl Sized;
                pub fn new<T>(t: T) -> ($crate::GCell<T, Opaque>, $crate::GToken<Opaque>) {
                    unsafe { $crate::GCell::manual_new(t, ()) }
                }
            }
            ctor::new(e)
        }
    }};
}
