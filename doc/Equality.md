# All Things Being Not Equal

Two objects are equal if they are completely undistinguishable. For example, `Alice` is indistinguishable from `Alice`, so they are equal words. Also, `indistinguishable` is indistinguishable from `indistinguishable`, so they are equal words, too.

Note that physical objects cannot be equal - they occupy different spaces, so they have at least one different property. However, if they are similar enough, they can be considered "equivalent".

We use equal words for equivalent objects.

For example:

| a | b |
|---|---|
| 1 | 1 |
| 2 | 2 |
| 3 | 3 |

Note that the rows `2 = 3`, `3 = 4`, `10 = 20` don't appear in this table.

Every valid row can be represented by an expression: `1 = 1`, `2 = 2`, `3 = 3`. However, we don't want to write each expression explicitly - there is an infinite amount of equalities, but we have finite memory in our devices. We can solve this problem by writing a generic expression that represents all equalities.

Notice that each equality matches a pattern: `x = x`. However, if we type this pattern directly, the compiler would treat `x` as a constant, not as a variable ([go deeper](ConstsVsVars.md)).
