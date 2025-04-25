```rust
match a {
    Foo(x, Bar(x)) => x,
    Foo(Bar(x), x) => x,
}

to
```

mplus(
    do {
        Foo(x, b) <- a,
        Bar(x) <- b,
        return x
    },
    do {
        Foo(b, x) <- a,
        Bar(x) <- b,
        return x
    }
)

