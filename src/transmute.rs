/// Transmutes between references. This function is safer than
/// casting between pointers because it keeps lifetimes intact.
/// This may be useful when creating FFI wrappers.
/// # Safety
/// - It must be sound to transmute T to U.
/// # Example
/// ```rust
/// # use wutil::transmute_ref;
/// #[repr(transparent)]
/// struct Foo(String);
///
/// impl AsRef<Foo> for String {
///     fn as_ref(&self) -> &Foo {
///         unsafe {transmute_ref::<String, Foo>(&self)}
///     }
/// }
///
/// let bar = "bar".to_owned();
/// let foo: &Foo = bar.as_ref();
///
/// assert_eq!(&foo.0, &bar);
/// ```
pub unsafe fn transmute_ref<T, U>(input: &T) -> &U { unsafe {
    (input as *const T).cast::<U>().as_ref().unwrap_unchecked()
}}

/// Transmutes between mutable references. This function is safer
/// than casting between pointers because it keeps lifetimes intact.
/// This may be useful when creating FFI wrappers.
/// # Safety
/// - It must be sound to transmute T to U.
/// # Example
/// ```rust
/// # use wutil::transmute_mut;
/// #[repr(transparent)]
/// struct Foo(String);
///
/// impl AsMut<Foo> for String {
///     fn as_mut(&mut self) -> &mut Foo {
///         unsafe {transmute_mut::<String, Foo>(self)}
///     }
/// }
///
/// let mut bar = "bar".to_owned();
/// let foo: &mut Foo = bar.as_mut();
///
/// foo.0.push('s');
///
/// assert_eq!(&bar, "bars");
/// ```
pub unsafe fn transmute_mut<T, U>(input: &mut T) -> &mut U { unsafe {
    (input as *mut T).cast::<U>().as_mut().unwrap_unchecked()
}}
