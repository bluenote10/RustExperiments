
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

# Design thoughts

How to represent?

```rust
view! { cx,
    <p>
    {move || if is_odd() {
        "Odd"
    } else {
        "Even"
    }}
    </p>
}
```

With the initial design this would be just:

```rust
comp!(
    p(
        move || if is_odd() {
            "Odd"
        } else {
            "Even"
        }
    )
)
```

But what about writing it like this?

```rust
comp!(
    move || if is_odd() {
        p("Odd")
    } else {
        p("Even")
    }
)
```

Should we support replacing element construction in arbitrary places in these expressions?
What if we accidentally replace something deep down in an expression that looks like element/component
construction, but isn't? For instance `p`/`div`/... could just be a variable/function in the calling
scope, and we should not mess with that. One way would be use the rule to only replace direct siblings
of the `comp!` macro. Since we pass `cx` and `style` implicitly this allows for some repetitive usage.
The second example would have to be written as:

```rust
comp!(
    move || if is_odd() {
        comp!(p("Odd"))
    } else {
        comp!(p("Even"))
    }
)
```

But somehow the issue with confusing an element/component construction with a variable still exists,
especially with a pattern like:

```rust
// pull out construction of some "div" for re-use
let div = comp!(...)

// now make use of it
comp!(
    div,                // not an element construction, variable reference
    p("sandwiched"),    // element construction
    div,                // not an element construction, variable reference
)

// alternatively
let div = || comp!(...)

// now make use of it
comp!(
    div(),              // not an element construction, (local) function call
    p("sandwiched"),    // element construction
    div(),              // not an element construction, (local) function call
)
```

This suggest that it would be good to have some kind of marker similar to RSX's tags that make
clear what is an element/component, and what not.

```rust
// now make use of it
comp!(
    e!(div),    // element construction
    div,        // variable reference
    div(),      // function call
)
```

A few syntactical variants:

```rust
comp!(
    e!(div, {a: b}, [child1, child2]),
    e!(div{a: b}(child1, child2)),
    e!(div{a: b}(child1, child2)),
    c!(MyComponent{a: b}(child1, child2)),
)
```

Whether disambiguating with `e!` and `c!` is sensible is questionable.
Somehow the inner macro calls make it obsolete to have an outer macro at all.
In this case it would only serve the purpose of abstracting over single element and
fragment expressions. This can probably be solved easier perhaps even just with an
array/vector.

```rust
// Size comparison vs RSX:
<p>"Hello world"</p>
c!(p("Hello world"))

<div>"Hello world"</div>
c!(div("Hello world"))
```
