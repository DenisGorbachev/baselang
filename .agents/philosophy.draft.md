# Philosophy

When we build systems, we start with a specification - a list of properties of the system functions.

A specification is complete if it contains all [essential properties](#essential-property) of the system.

A data structure is "bytes with laws" - a list of functions that satisfy certain properties.

The core expectation is determinism. We define a fully deterministic system as "for any data and any address, a write of this data at this address followed by read at this address will return the same data". Of course, real systems are not deterministic. Our goal as engineers is to build systems that are as deterministic as possible and document as many cases of non-determinism as possible.

A system specification must also list "negative laws" - laws that are true for another similar system but not this one.

A system specification should also list non-requirements and non-preferences.

Surprisal is a count of negative laws divided by total count of laws (TODO: think about how to define surprisal between two systems, not just one - they have different counts of laws).

"Constraints" are propositions that state non-existence of certain values of a type (for example: non-existence of certain states, like Pauli exclusion principle).

## Definitions

### Specification

A list of functions whose input contains an old state or whose output contains a new state.

Examples:

* Map
  * `get : (K V : Type) -> (m : Map K V)`
  * `insert : (K V : Type) -> (key : K) -> (value : V) -> (old : Map K V) -> (new : Map K V)`

Notes:

* The "input contains state" and "output contains state" are defined according to [Type A contains Type B](#type-a-contains-type-b)
* Input may contain other types besides state
* Output may contain other types besides state (e.g. errors)

### Program

TODO

### Active program

A [program](#program) that may call other programs without itself being called by another program.

Examples:

* A trading bot

### Passive program

A [program](#program) that may call other programs only as a result of itself being called by another program.

Examples:

* An exchange

### Essential property

A property that can't be proven from other properties.

Notes:

* Since derivation is equivalent to proving this property as a theorem, an essential property can also be called an "axiom".

### Type A contains Type B

Type A contains Type B if every constructor of Type A has at least one argument of Type B.
