# Temporary Annex

[![](https://img.shields.io/crates/v/temporary-annex)](https://crates.io/crates/temporary-annex)

Allows pushing to a `Vec` (or any other type implementing `Annexable`) with cleanup once result goes out of scope.

e.g.

```rust
let mut vec1 = vec![1, 2, 3];
assert_eq!(vec1, [1, 2, 3]);
{
    let new_vec_ref = vec1.push_annex(4);
    assert_eq!(*new_vec_ref, [1, 2, 3, 4]);
}
assert_eq!(vec1, [1, 2, 3]);
```

This has the effect of a immutable structure but uses the same underlying allocation (no cloning or additional allocations).
