# Definitions

## Character

Any printable non-whitespace Unicode character.

Examples:

* `a`
* `å`
* `β`

Counter-examples:

* Space character
* Tab character
* Linefeed character

## Space

The " " [character](#character) (U+0020).

## Word

A non-empty list of [characters](#character) without [spaces](#space).

## Expression

A non-empty list of [words](#word).

Notes:

* When rendered, the words are separated by spaces.

## Earlier variable

Variable `A` is said to be "earlier" than `B` if it is defined at the same level but before `B`.

* Earlier variable is an inverse of [later variable](#later-variable)

## Later variable

Variable `A` is said to be "later" than `B` if it is defined at the same level but after `B`.

* Later variable is an inverse of [earlier variable](#earlier-variable)
