# Baselang

Baselang is a language for writing proofs. If you need to prove your ideas (to yourself, to other people), you can use Baselang to automatically check these proofs. This way, you can convince yourself and others that you are right.

To prove an idea, you need to:

1. Express your idea.
2. Ask an LLM to write a proof.
3. Ask the Baselang compiler to check the proof.

Then you can send your "idea & proof" to other people. They can check it themselves to become convinced.

Baselang doesn't fully eliminate the arguments between people, but it focuses the discussions on a small set of common axioms. If you agree on the axioms, then you automatically agree on the theorems (the conclusions drawn from the axioms).

## Compiler

The Baselang compiler is a program that reads Baselang files. It treats every file as a "todo list" (a sequence of commands). For example:

```baselang
@print("Hello")
```

The compiler reads the first line, executes it by printing "Hello", then exits.

In this example:

- `@print` is a compiler command name.
- `"Hello"` is a compiler command parameter.
- `@print("Hello")` is a compiler command expression.

A compiler command expression always contains a name and zero-or-more parameters.

## Memory

The `@print` command instructs the compiler to output some text. But the compiler doesn't remember this text after outputting it. You can tell the compiler to memorize it:

```baselang
text = "Hello"
@print(text)
@print(text)
```

After reading this file, the compiler outputs "Hello" twice.

## Comments

Sometimes, we need to write the notes for people who would read the Baselang files. These notes should not be executed by the compiler. We can use `//` for single-line comments:

```baselang
text = "Hello"   //   `text` becomes "Hello"
@print(text)     //   prints "Hello"
@print(text)     //   prints "Hello"
```

We can also use paired `/*` and `*/` for multi-line comments:

```baselang
text = "Hello"
@print(text)
@print(text)
/*
 
  These commands print two lines:
 
  Hello
  Hello
 
*/ 
```

## Forms

Every compiler command has a general form. For example:

- `@print` command has a general form `@print($value)`
- `=` command has a general form `$name = $value`

In these examples, `$name` and `$value` are placeholders that can be substituted for actual names and values.

Notice that `$name` must be a name - it can't be a number, it can't be a string. For example:

- `2 = "Hello"` is invalid
- `"H" = "Hello"` is invalid

Invalid commands are rejected by the compiler because it doesn't know how to execute them. It could, theoretically, execute `2 = "Hello"` by memorizing that `2` becomes `"Hello"`, but that would break `2 * 2`, which would become `"Hello" * "Hello"`. We could, theoretically, define a string multiplication, but - important - we deliberately choose not to define it, because we **want** the compiler to reject invalid commands. Invalid commands become "falsities", "heresies", "untruths".

Thus, valid commands become "truths", and valid programs become "true". Notice how elegant this definition is:

- An empty program is true: it doesn't contain any contradictory statements.
- An empty compiler (imagine a compiler without any builtin commands) accepts an empty program but rejects every other program: it doesn't trust anything we tell him.
- A compiler trust surface can be extended by defining more commands.
- A compiler trust surface can be limited by restricting the list of commands to the bare minimum (for example: allowing `*` for numbers but disallowing it for strings).

## New commands

To use the compiler to check our ideas, we need to define our own commands. We'll start with a simple example:

```baselang
print_hello_twice = ```
@print("Hello")
@print("Hello")
\`\`\`
```

This defines `print_hello_twice` as a code block that contains two commands. It doesn't execute the commands yet. To actually execute the commands in a code block, we need a separate `@evaluate` command:

```baselang
@evaluate(print_hello_twice)
```

This will actually print "Hello" twice.

## Natural numbers

What if we want to print "Hello" any number of times (not exactly two)? For that, we need to define a new command `@print_many`. This command must take a parameter `n` to indicate how many times to print. `n` is a natural number. Thus, to define this parameter, we need to define the natural numbers first.

We start with an observation that "natural number" is a type. We'll use a shorthand `Nat` to denote natural numbers:

```baselang
Nat : Type   //   Nat is Type
```

We continue with another observation that 0 is a natural number:

```baselang
0 : Nat   //   0 is Nat
```

We could continue with `1 : Nat`, `2: Nat`, `3 : Nat` ... - but there are infinitely many natural numbers. That means we need to find a solution to a problem: how to define infinitely many values with finitely many lines of code?

The solution comes from another observation: that every next natural number is one step ahead of the previous natural number. For example:

- `1` is one step ahead of `0`
- `2` is one step ahead of `1`
- `3` is one step ahead of `2`
- ...

So `Step` definitely exists, but what type does it have?

```baselang
Step : ?    //    Step is ... (what?)
```

Notice that `Step` is not an object - it's a process. It transforms the previous natural number into the next natural number.

Let's use the symbol `->` to denote a process. Its general form will be `$source -> $target` (alternatively: `$from -> $to`, `$origin -> destination`, `$start -> $finish`).

For `Step`, this will be `(prev : Nat) -> (next : Nat)`. So the whole definition would be:

```baselang
Step : (prev : Nat) -> (next : Nat)   //   Step is a process that transforms prev Nat into next Nat 
```

Then we can define `1`, `2`, `3`, ... using only `0` and `Step`:

```baselang
1 = Step(0)    //    1 becomes Step(0)
2 = Step(1)    //    2 becomes Step(1)
3 = Step(2)    //    3 becomes Step(2)
```

Notice that `1`, `2`, `3` are just synonyms. The only fundamentally new names are `0` and `Step`.

We could continue using `0` and `Step` directly. But it's not particularly elegant since `0` looks like a number, while `Step` looks like a word. To improve on elegance, we can use the word `Zero` instead of `0`. Then the full definition becomes:

```baselang
Nat : Type                            //   Nat is a type
Zero : Nat                            //   Zero is a Nat
Step : (prev : Nat) -> (next : Nat)   //   Step is a process that transforms prev Nat into next Nat

0 = Zero
1 = Step(0)
2 = Step(1)
3 = Step(2)
```

Elegant, simple, beautiful.
