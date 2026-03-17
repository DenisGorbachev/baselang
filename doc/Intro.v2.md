# Baselang

Baselang is a language for fighting AI slop.

Slop is a text riddled with ambiguities. For example, "war is good" is sloppy because it doesn't define "war" or "good". For another example, "war is bad" is just as bad.

Baselang can also detect logical contradictions. For example:

```text
`War` is a concept
`Good` is a concept
`Bad` is a concept

`War is good` is a path from `War` to `Good`
`War is bad` is a path from `War` to `Bad`

`A concept can't be both good and bad` is a path
  from
    `A` as a concept,
    `A is good` as a path from `A` to `Good`,
    `A is bad` as a path from `A` to `Bad`
  to insanity
```

Baselang output:

```text
Error: found 1 proof of void.

- `A concept can't be both good and bad` of `War`, `War is good`, `War is bad`
```

You can think of `A concept can't be both good and bad` as a function of three arguments. When called, it returns "insanity", a special type that indicates a logical contradiction. Here's the same example in other languages:

```rust
fn a_concept_cant_be_both_good_and_bad<A>(
    a_is_good: fn(A) -> Good,
    a_is_bad: fn(A) -> Bad
) {
    panic!()
}
```

```typescript
function aConceptCantBeBothGoodAndBad<A>(
  aIsGood: (a: A) => Good,
  aIsBad: (a: A) => Bad
): never {
  throw new Error("unreachable");
}
```

```python
A = TypeVar("A")

def a_concept_cant_be_both_good_and_bad(
    a_is_good: Callable[[A], Good],
    a_is_bad: Callable[[A], Bad],
) -> NoReturn:
    raise RuntimeError("unreachable")
```

Note that Baselang needed an additional assumption (`A concept can't be both good and bad`). This is because Baselang doesn't assume anything about "Good" or "Bad" (or any other concepts). Any relations between the concepts must be defined explicitly.
