# unbytes
Ergonomic, performant, owned forward-only cursors based on `bytes`, with some bonus features.

`unbytes` gives the following guarantees:
- Never panics.
- Never copies.
- Never allocates.

Note that implementations involving the `bytes` crate *can* allocate. Traits like `Into<Bytes>`, especially on `Vec`s, are very likely to reallocate. `unbytes` can't do anything about that.

The following feature flags are available, but none are enabled by default.
- `std`: Standard library support, like an `std::io::Read` implementation.
- `maypanic`: Functionality that **may** panic if used improperly, like a `Buf` implementation.