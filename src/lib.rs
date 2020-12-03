/*!
    Yet another encoding/decoding library for bencode.

    I believe [`deserializer`](crate::Deserializer) don't allocate memory. [`Serializer`](crate::Serializer) allocates
    only when `sort_dictionary` feature enabled. Must be fast enough.

    ## Examples
    See

    ## Caveats
    `serde` treats `[u8; N]`, `Vec<u8>`, `&[u8]` like any other
    sequence, that is it will be encoded as list of bytes(not a byte string).

    Sollution - use [`serde_bytes`](https://github.com/serde-rs/bytes).
    ## Behaviour
    This crate does sort dictionary by keys if `sort_dictionary` feature is
    enabled (enabled by default). Otherwise order of elements can vary
    (depends on [`Serialize`](serde::Serialize) trait implementation).

    ### Mapping of rust types to bencode
    - [`bool`](bool) is [`integer`](#integers) either `i1e` or `i0e`.
    - `all primiteve integer types` is [`integer`](#integers).
    - [`char`](char) is [`byte string`](#byte-strings) with length at most 4 bytes.
    - [`String`](String) is [`byte string`](#byte-strings).
    - `[u8]` is [`list`](#lists) see [`Caveats`](#caveats).
    - [`Option`](Option) is `bencode value` if `Some` otherwise nothing will be written.
    - `()` (unit) is `0:`, empty [`byte string`](#byte-strings).
    - `struct UnitStruct;` is `10:UnitStruct`, [`byte string`](#byte-strings) containing name
    of the unit struct.
    - `enum E { A, B }`
        - `E::A` is `d1:E1:Ae`, [`dictionary`](#dictionaries) with one entry. Key is name of
        the `enum` and value is the variant.
    - `struct Wrapper(u8)` (newtype struct) is bencode of the containing type.
    - `tuple`s and `array`s is [`lists`](#lists), tuples can be heterogeneous.
    - `struct Rgb(u8, u8, u8)` (tuple struct) is [`list`](#lists) of tuple values.
    - [`HashMap`](std::collections::HashMap) is [`dictionary`](#dictionaries).
    - `struct`s is [`dictionary`](#dictionaries). Keys are field names, values are
    field values.

    - [`f32`](f32), [`f64`](f64) is not supported.

    ## Alternatives
    Arbitrary order. Search more on [`crates.io`](https://crates.io/search?q=bencode) or
    [`lib.rs`](https://lib.rs/search?q=bencode).

    - [`bendy`](https://lib.rs/crates/bendy)
    - [`bencode`](https://lib.rs/crates/bencode)
    - [`serde_bencode`](https://lib.rs/crates/serde_bencode)
    - [`bt_bencode`](https://lib.rs/crates/bt_bencode)
    - [`bip_bencode`](https://lib.rs/crates/bip_bencode)

    ## Bencoding
    ### Supported data types
    - [`Byte Strings`](#byte-strings)
    - [`Integers`](#integers)
    - [`Lists`](#lists)
    - [`Dictionaries`](#dictionaries)
    #### Byte Strings
    `<base ten ASCII>:<string bytes>`

    Examples:
    - `4:abcd`
    - `0:`

    #### Integers
    `i<base ten ASCII, optional minus sign>e`

    The maximum number is not specified. This crate
    handles integers as [`u64`](u64) or [`i64`](i64).

    Examples:
    - `i123456e`
    - `i-5e`

    Malformed:
    - `i03e`, anything that starts with 0, except `i0e`
    - `i-0e`

    #### Lists
    `l<bencode type values>e`

    Examples:
    - `li0ei1ei2ee` == `[1,2,3]`
    - `le` == `[]`

    #### Dictionaries
    `d<bencoded string><bencoded element>e`

    Keys must be sorted as __raw__ strings. [`string`](#byte-strings)'s should be
    compared using a __binary comparison__.

    Examples:
    - `de` == `{}`
    - `d4:rustl2:is7:awesomeee` == `{"rust" => ["is", "awesome"]}`

    ## Crate features
    ### sort_dictionary
    Enables sort by keys when serializing to bencode dictionary.
*/

mod de;
mod error;
mod ser;

// pub use de::{from_str, Deserializer};
pub use de::{from_bytes, from_str, Deserializer};
pub use error::{DeError, DeResult, SerError, SerResult};
pub use ser::{to_string, to_vec, to_writer, Serializer};
