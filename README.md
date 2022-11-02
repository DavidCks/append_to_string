## append_to_string rust macro

<p>
  <a href="https://github.com/DavidCks/append_to_string/actions?query=branch%3Amain">
    <img src="https://github.com/DavidCks/append_to_string/workflows/Rust%20CI/badge.svg"
         alt="Rust CI badge">
  </a>
</p>

The append_to_string!() macro appends every occurance of a literal within a struct or on its own with a .to_string(), converting it to a string.
The literals type must implement the .to_string() method.

* converting a literal

```rust
// str example
let a = "value".to_string();
let b = append_to_string!("value");
assert_eq!(a, b);

// int example
let a = 42.to_string();
let b = append_to_string!(42);
assert_eq!(a, b);

```

* converting a Struct

```rust

// structs
struct A {
    s1: String,
    s2: String,
}

struct B {
    s1: String,
    s2: String,
    a: A,
}

// simple struct example
let a1 = append_to_string!( 
    A {
        s1: "hello",
        s2: "world", 
    }
);

let a2 = A {
    s1: "hello".to_string(),
    s2: "world".to_string(), 
};

assert_eq!(a1, a2);

// nested struct example
let b1 = append_to_string!( 
    B {
        s1: "hello",
        s2: "world",
        a: A {
            s1: "nested",
            s2: "struct",
        }
    }
);

let b2 = B {
    s1: "hello".to_string(),
    s2: "world".to_string(),
    a: A {
        s1: "nested".to_string(),
        s2: "struct".to_string(),
    }
};

assert_eq!(b1, b2);

```

May be useful for when you need to create big structs with String fields but want to keep the code readable or save time by not typing out a conversion method for &str types.

## dependencies

this crate uses the syn, quote, and proc_macro2 crates.


