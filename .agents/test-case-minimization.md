# Test case minimization

## Principles

* Preserve the test semantics.
* Prefer `var!`, `typ!`, `exp!`:
  * Examples:
    * `var!(x)`
    * `var!(x: typ!(&a))`
* Prefer the shortest constructor or macro that expresses the intended AST.
  * Examples:
    * `typ!(y => &x)` instead of `Fun(y.clone(), Box::new(One(Sol(x.clone()))))`
    * `typ!(&a)` instead of `typ!(Sol(a.clone()))`
    * `Exp::sol(&y)` instead of `Sol(y.clone())`
* Prefer `From` / `TryFrom` / `Into` / `TryInto`.
* Keep expected values independent from the code under test.
  * In substitution tests, avoid `exp!` because it produces code that calls `substitute` internally.
