## Hearing

It's important to hear programs - to actually hear the sounds in your imagination while you're reading the program. For example, `@print("Hello")` can be heard as `Print "Hello"` (pretty natural).

But how would you hear `text = "Hello"`? My proposal is `text becomes "Hello"`. This expresses the idea that every occurrence of `text` can be replaced with `"Hello"`, and the program output will be the same (two "Hello" lines).

---

Most compiler commands have descriptive names like `@print`. But some commands are used very often, so they have been shortened to single symbols:

**Important**: the `=` symbol is a compiler command name, too. It takes two parameters: the name `text` and the string `"Hello"`. So writing `text = "Hello"` is equivalent to writing `=(text,  "Hello")`.

---

`$name = $value` creates a synonym of existing value. To create a new value, we need a "constructor".

---

## Types

`$name = $value` command creates synonyms of existing values. For creating new values, there is another command: `$name : $type`. For example:

```baselang
Human : Type
Alice : Human
Bob: Human
```

Notice that `Alice` and `Bob` don't have definite values. This is logically correct because they are humans, and we don't know the exact values of humans.
