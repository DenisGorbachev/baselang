We want to render the code in natural language. By Curry-Howard correspondence, every type is a proposition, so we can render any Var in a natural language.

* Var name can be singular or plural
* Var name can be uppercase or lowercase
* Var name must be static (can't be a function) because eventually we'll store it in the database
* Var name may contain references to other fields to minimize the memory requirements

```rust
pub enum Plural {
  Custom(String),
  SingularWithSuffix(Suffix)
}

pub enum Suffix {
  S,
  ES
}
```

This may not be necessary initially.

Same for capitalization / camel-calization.

* Some vars have equal uppercase and lowercase names (e.g. abbreviations (e.g. "MIT"))
* Some var names may contain non-ASCII symbols (e.g. "Saint-Germain-des-Prés")
* Some var names don't make sense in plural form (e.g. "Moscow" doesn't make sense in plural form)
* We need to implement also the conversion to CamelCase and snake_case (but these should follow specific pre-defined rules)
  * Some var names have non-equal natural singular names but equal CamelCase singular names (e.g. "Saint-Germain" vs "Saint Germain")
    * This may pose an issue in round-tripping the representations
* Some pairs of theories may contain vars with equal names
  * We need to support renaming when importing
* Users should be able to contribute the name packs without permission of the var owner
  * Is there even such a thing as a var owner? Since the code is immutable, the only thing that is important is "bundling" vars and names (defining namespaces).
  *
* Users should be able to create theories (import only specific vars, thus limiting the list of constructors)
* Users should be able to extend theories (import existing theories + add their own vars)
  * Initially: just fork the theories (we'll be able to back-merge them if necessary)
