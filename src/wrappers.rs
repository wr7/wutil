use core::{
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};

/// A wrapper that guarentees that the dropper of the underlying object cannot be safely called.
#[repr(transparent)]
pub struct DontDrop<T>(ManuallyDrop<T>);

impl<T> DontDrop<T> {
    pub const fn new(value: T) -> Self {
        Self(ManuallyDrop::new(value))
    }

    /// Gets the underlying value.
    /// # Safety
    /// At least one of the following must be upheld
    /// - The inner value cannot be dropped
    /// - The circumstances outlined in the safety section of [DontDrop::drop]
    pub const unsafe fn into_inner(self) -> T {
        core::mem::ManuallyDrop::into_inner(self.0)
    }

    /// Drops the underlying value.
    /// # Safety
    /// Documentation from the provider of this value may specify
    /// circumstances where this function is safe to call.
    pub unsafe fn drop(self) {
        core::mem::drop(core::mem::ManuallyDrop::into_inner(self.0));
    }
}

impl<T> From<T> for DontDrop<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> AsRef<T> for DontDrop<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for DontDrop<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Deref for DontDrop<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for DontDrop<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
