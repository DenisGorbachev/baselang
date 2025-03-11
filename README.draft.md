# Baselang

Baselang is a language for describing how things work.

Features:

* [Dependent types]
* [Multiple renderers](#renderers) (see: [Rust-like](#rust-like), [TypeScript-like](#typescript-like), [Haskell-like](#haskell-like))
* [Exploration mode](#exploration-mode)
* ... and many [minor conveniences](#conveniences)

Baselang is written _in Rust_ and _within Rust_. Here's an example:

```rust
pub struct Bool {
    pub bool: Rc<Var>,
    /// This field is called `yes` to avoid conflict with Rust `true`
    pub yes: Rc<Var>,
    /// This field is called `no` to avoid conflict with Rust `false`
    pub no: Rc<Var>,
}

impl Default for Bool {
    fn default() -> Self {
        // Bool : Type
        let bool = Var::new_top_rc("Bool");

        // Yes : Bool
        // This variable is named `yes` instead of `true` because `true` is a reserved keyword in Rust
        let yes = Var::new_rc("True", Typ::One(Exp::Sol(bool.clone())));

        // No : Bool
        // This variable is named `no` instead of `false` because `false` is a reserved keyword in Rust
        let no = Var::new_rc("False", Typ::One(Exp::Sol(bool.clone())));

        Self {
            bool,
            yes,
            no,
        }
    }
}
```

Since Baselang is written within Rust, you can use Cargo and crates.io for package management. Also, you can write the code with full IDE support: autocompletion, syntax highlighting, etc. If you'd like to minimize the size of codebase, you can use [Baselang-specific macros](#macros).

## Renderers

Baselang code can be displayed in multiple styles. This improves readability for people who are more familiar with a certain style (for example, Rust programmers would prefer reading Baselang rendered in Rust style).

### Rust-like

### TypeScript-like

### Haskell-like

## Exploration mode

_This is a planned feature._

## Conveniences

* [Multi-named variables](#multi-named-variables)

### Multi-named variables

### Macros

Use [`var!`](./src/types/var.rs), [`typ!`](./src/types/typ.rs), [`exp!`](./src/types/exp.rs) to minimize the codebase size.
