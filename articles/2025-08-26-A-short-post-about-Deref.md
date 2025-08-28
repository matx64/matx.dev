---
title: "A short post about Deref"
date: "Aug 26, 2025"
---

`Deref` is one of my favourite Rust traits for keeping the code clean. The [official docs](https://doc.rust-lang.org/std/ops/trait.Deref.html) describe it like this:

> Used for immutable dereferencing operations, like `\*v`.
>
> In addition to being used for explicit dereferencing operations with the (unary) `\*` operator in immutable contexts, `Deref` is also used implicitly by the compiler in many circumstances. This mechanism is called “Deref coercion”. In mutable contexts, `DerefMut` is used and mutable deref coercion similarly occurs.

I mean... it's a technical documentation, but I still think it's a mouthful. In practice, `Deref` is straightfoward: it basically let's custom types behave like references to their inner data, providing great ergonomics to the codebase.

Imagine a tiny `Document` type which is basically a wrapper around `String`. This type was created for extra validation, as Document isn't just a generic string. Even though the type is tiny, for most programming languages now you have to deal with getters and general unwrapping to access the inner String value.

Luckily, `Deref` solves this problem:

```rust
pub struct Document(String);

impl Document {
    pub fn new(val: impl Into<String>) -> Result<Self, &'static str> {
        // .. perform validation
        Ok(Self(val.into()))
    }
}

impl std::ops::Deref for Document {
    type Target = str;
    fn deref(&self) -> &Self::Target { &self.0 }
}
```

`Deref` now let's us pass `&Document` where `&str` is expected and even call `str` methods without inner `String` being public or using `.0` access:

```rust
fn mask_sensitive_data(s: &str) -> String {
    // .. mask str
    "*****".into()
}

fn main() {
    let doc = Document::new("passport-123").unwrap();

    // Deref allows passing &Document where &str is expected:
    let masked = mask_sensitive_data(&doc);
}
```

That's it! This trait is great when your type is essentially a smart pointer / wrapper and you want transparent access (method calls, indexing, coercion to &Target). Implementing this trait makes the custom type feel like a native.
