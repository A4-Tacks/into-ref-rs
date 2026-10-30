Generic assist traits, consume self into a reference

| Input     | `Into<&T>`    | `Into<&mut T>`  | `IntoRef<Out = T>`  | `IntoMut<Out = T>`  |
| `&T`      | pass          | fail            | pass                | fail                |
| `&mut T`  | fail          | pass            | pass                | pass                |

Examples
-------------------------------------------------------------------------------

AsRef/AsMut cannot using in owned convert

```rust,compile_fail
fn foo<'a>(i: impl IntoIterator<Item: 'a + AsRef<u32>>) -> Vec<&'a u32> {
    i.into_iter().map(|it| it.as_ref()).collect()
}
fn bar() { foo([&2u32]); } // compile_fail
fn baz() { foo([&mut 2u32]); } // compile_fail
```

> error[E0515]: cannot return value referencing function parameter `it`

-------------------------------------------------------------------------------

Use `Into` to instead, cannot convert `&mut T -> &T`:

```rust,compile_fail
fn foo<'a>(i: impl IntoIterator<Item: Into<&'a u32>>) -> Vec<&'a u32> {
    i.into_iter().map(Into::into).collect()
}
fn bar() { foo([&2u32]); } // pass
fn baz() { foo([&mut 2u32]); } // compile_fail
```

> error[E0277]: the trait bound `&u32: From<&mut u32>` is not satisfied

-------------------------------------------------------------------------------

Use `IntoRef` to instead:

```rust
use into_ref::IntoRef;
fn foo<'a>(i: impl IntoIterator<Item: IntoRef<'a, Out = u32>>) -> Vec<&'a u32> {
    i.into_iter().map(IntoRef::into_ref).collect()
}
fn bar() { foo([&2u32]); } // pass
fn baz() { foo([&mut 2u32]); } // pass
```
