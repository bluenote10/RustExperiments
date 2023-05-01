
# What does rustfmt support?

The semantics are unclear to me. It looks like using `{}` braces leads to not trying to do anything with the macro.
This is probably a result of [this line](https://github.com/rust-lang/rustfmt/blob/a44c7ea5923caa8f908ae0fdd6563033a7ad88da/src/parse/macros/mod.rs#L106).

For parens or bracket based argument lists, rustfmt seems to support `Expr`, `Ty`, `Pat`, and `Item`.

Note that rustfmt still does _something_ with curly braces macros. It looks like looks for the indentation
of the closing brace, and re-indents the macro content so that the closing brace is on the right indentation
level. For example:

```rust

    // This
    comp! {
    foo
};

    // becomes
    comp! {
        foo
    };

    // This
    comp! {
foo
};

    // becomes
    comp! {
    foo
    };

    // This
    comp! {
                foo
        };

    // becomes
    comp! {
            foo
    };
```