#![doc = include_str!("../README.md")]

/// Derives the [`FieldNamesAsArray`] trait.
///
/// # Panics
///
/// If the token stream is not coming from a named struct or if
/// the `field_names_as_array` attribute is used wrongfully, deriving
/// this macro will fail.
///
/// # Examples
///
/// ```
/// use struct_field_names_as_array::FieldNamesAsArray;
///
/// #[derive(FieldNamesAsArray)]
/// struct Foo {
///     bar: String,
///     baz: String,
///     bat: String,
/// }
///
/// assert_eq!(Foo::FIELD_NAMES_AS_ARRAY, ["bar", "baz", "bat"]);
/// ```
///
#[cfg_attr(doc_cfg, doc(cfg(feature = "derive")))]
pub use struct_field_names_as_array_derive::FieldNamesAsArray;

/// Derives the [`FieldNamesAsSlice`] trait.
///
/// # Panics
///
/// If the token stream is not coming from a named struct or if
/// the `field_names_as_array` attribute is used wrongfully, deriving
/// this macro will fail.
///
/// # Examples
///
/// ```
/// use struct_field_names_as_array::FieldNamesAsSlice;
///
/// #[derive(FieldNamesAsSlice)]
/// struct Foo {
///     bar: String,
///     baz: String,
///     bat: String,
/// }
///
/// assert_eq!(Foo::FIELD_NAMES_AS_SLICE, ["bar", "baz", "bat"]);
/// ```
///
#[cfg_attr(doc_cfg, doc(cfg(feature = "derive")))]
pub use struct_field_names_as_array_derive::FieldNamesAsSlice;

/// Exposes the `FIELD_NAMES_AS_ARRAY` constant.
///
/// This trait is designed to be derived rather than implemented by
/// hand (though that'd be perfectly fine as well).
/// Please refer to the [top-level](crate) documentation for more
/// information.
///
pub trait FieldNamesAsArray<const N: usize> {
    const FIELD_NAMES_AS_ARRAY: [&'static str; N];
    const FIELD_TYPES_AS_ARRAY: [&'static str; N];
}

/// Exposes the `FIELD_NAMES_AS_SLICE` constant.
///
/// This trait is designed to be derived rather than implemented by
/// hand (though that'd be perfectly fine as well).
/// Please refer to the [top-level](crate) documentation for more
/// information.
///
pub trait FieldNamesAsSlice {
    const FIELD_NAMES_AS_SLICE: &'static [&'static str];
    const FIELD_TYPES_AS_SLICE: &'static [&'static str];
}
