/// Defines a `&'a static CStr` from a byte string literal. This does not compile if
///  an invalid CString is supplied.
/// ```rust
/// # use std::ffi::{CStr, CString};
/// # use wutil::cstr;
/// const FOO: &CStr = cstr!(b"ptooey\0");
/// let foo = CString::new("ptooey").unwrap();
///
/// assert_eq!(FOO, foo.as_ref());
/// assert_eq!(cstr!(b"ptooey\0"), foo.as_ref());
/// ```
/// The following examples fail to compile.
/// ```compile_fail
/// # use std::ffi::{CStr, CString};
/// # use wutil::cstr;
/// const BAR: &CStr = cstr!("this is not a byte string literal\0");
/// ```
/// ```compile_fail
/// # use std::ffi::{CStr, CString};
/// # use wutil::cstr;
/// const BAR: &CStr = cstr!(b"this is missing a null byte");
/// ```
/// ```compile_fail
/// # use std::ffi::{CStr, CString};
/// # use wutil::cstr;
/// const BIZ: &CStr = cstr!(b"this contains multiple null bytes\0\0");
/// ```
/// ```compile_fail
/// # use std::ffi::{CStr, CString};
/// # use wutil::cstr;
/// const BANG: &CStr = cstr!(b"this has a null byte\0 in the middle");
/// ```
#[macro_export]
macro_rules! cstr {
    ($string:literal) => {{
        const TMP: &::core::ffi::CStr = {
            if let Ok(s) = ::core::ffi::CStr::from_bytes_with_nul($string) {
                s
            } else {
                panic!("Invalid CString. Input must have exactly one null byte and it must be at the end of the string.")
            }
        };

        TMP
    }};
}
