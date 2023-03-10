Snippet that demonstrate const evaluation at compile time, by adding constraints on a const variable.

It uses nightly incomplete feature `generic_const_exprs`.

Test: uncomment the line in `main.rs` and it will fail to compile with `cargo check`.

Resources:
- https://github.com/rust-lang/rust/issues/76560
- https://rust-lang.zulipchat.com/#narrow/stream/260443-project-const-generics/
