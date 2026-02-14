# General

## Project

A list of files that can be used for at least two purposes:

* To produce at least one [product](#product).
* To produce the next version of the project itself.

Notes:

* The project may contain program files and [auxiliary files](#auxiliary-file).

## Product

A file or directory that helps the [user](#user) achieve his goal.

Examples:

* A CLI program
* A Rust crate

Notes:

* This definition is intentionally narrow.

## Auxiliary file

A file with the following properties:

* Belongs to a [project](#project)
* Doesn't contain a program
* Is read by the agent or a program that the agent invokes while working on the project

Examples:

* Package manager config
* Linter config
* Agent context file

## High-level specification

A string that can be interpreted as a struct with the following fields:

* `sources` (a list of source APIs)
* `target` (a single target API)
* `audience` (a string that describes a list of [users](#user))

Examples:

* "A Fjall database CLI for developers"
  * `sources` includes the Fjall API and the filesystem API
  * `target` includes the executable API (stdin, stdout, stderr) and the shell API (e.g. escape codes)
  * `audience` includes the developers

Notes:

* The "source API", "target API", "description" are not defined yet.
* The example for "Types that provide compile-time safety for arithmetic operations and conversions between units-of-measurement" doesn't match the proposed formalization

## Implementation

Preferences:

* Should not use [optimization hacks](#optimization-hack).

## Constructor of type T

A function whose return type is exactly `T`.

Notes:

* Every constructor is a [producer](#producer-of-type-t)

## Producer of type T

A function whose return type mentions `T`.

Examples:

* `fn foo(a: A, b: B) -> Result<K, M>` is a producer of `Result`, `K`, `M`

## Producing expression of type T

An expression whose return type mentions `T`.

Synonyms: ProdExp.

Examples:

* `UserBuilder::default().name("Alice").build()?` is a ProdExp of `User`

Requirements:

* Must set a value for every field that can be set
  * Should use the most explicit constructor

Preferences:

* Should rely on `impl Into` to reduce the code size
  * Examples:
    * `Article::new`
      * Constructor: `impl Article { pub fn new(title: impl Into<String>, text: impl Into<String>) -> Self { ... } }`
      * ProdExp:
        * Bad: `Article::new("Title".to_string(), "Text".to_string())` (bad because it explicitly converts str to String using to_string)
        * Good: `Article::new("Title", "Text")` (good because it relies on `impl Into`)

## Parameter of a producing expression of type T

A variable that is passed into the [producing expression of type T](#producing-expression-of-type-t).

## External-facing fn

A Rust fn that calls an [external API](#external-api).

Preferences:

* Should be [reversible](#reversible-fn), unless it is expected by the user to be irreversible
  * Examples of fns that are expected to be irreversible:
    * `FileDeleteCommand::run` (the user explicitly types "delete" when invoking this command)
  * Examples of fns that are expected to be reversible:
    * `FileShowCommand::run` (the user does not type any destructive words when invoking this command)

## External API

An API that the program uses to read the input or write the output.

Examples:

* Terminal emulator API
* Shell API
* Browser API
* Filesystem API (can be either external or internal)

## Internal API

An API that the program uses to read or write the parts of the state which are not an explicit input or output.

Examples:

* Filesystem API (can be either external or internal)
* Database API

## Reversible fn

A Rust fn whose effect can be reversed.

Properties:

* Some fns that take arguments only by reference are irreversible
  * Example:
    * `remove_file` that takes a `&Path` and removes the file
* Some reversible fns call irreversible fns
  * Example:
    * `remove_file_with_backup` that makes a backup of a file before removing it

Notes:

* Formal definition: Rust fn `f` is reversible if there exists a Rust fn `g` that takes the output of `f` as input so that when `f` and `g` are executed sequentially the [extended state](#extended-state) of the program is not modified.

## Irreversible fn

A Rust fn that is not [reversible](#reversible-fn).

## Extended state

A state that contains all data that the program can read (including memory, disks, databases, remote APIs).

## Input source

The original location of the data that is held by the input variable (where the program has read it from).

Examples:

* Configuration file
* HTTP request
* Program arguments (`argv`)
* Standard input (`stdin`)
* Environment (`env`)

## Total order on projects

Project A is better than Project B if it has a lower total loss.

The total loss is calculated in the following way:

* Make a list of properties of the population of users.
  * Make a list of resources that the users possess that are relevant to the program (see [software resources](#software-resources)).

Notes:

* The order is defined on projects instead of programs because we actually build projects, not programs.

## Software resources

TODO

* Processor:
  * CPU (speed)
  * GPU (speed)
  * Remote server processor (speed)
* Memory:
  * RAM (capacity, speed)
  * GPU Memory (capacity, speed)
  * Disk (capacity, speed)
  * Remote server storage (speed)
* Rate limits on external API

## Agent

An entity that is assumed to be working towards a specific goal.

Notes:

* This definition is intentionally broad.

## User

An [agent](#agent) that uses an [product](#product) to work towards its goal.

## Optimization hack

An optimization that relies on a property that cannot be assumed to hold in the future.

Examples:

* An optimization that relies on an implementation detail in a dependency (this property cannot be assumed to hold in the future because the implementation details are not a part of the public interface and may change without notice).

---

Write code to minimize losses.

Kinds of losses:

* External losses incurred by users
* Internal losses incurred by developers

Types of losses:

* Unavoidable losses (costs).
* Avoidable losses (mistakes).

The most fundamental principle of software development is to minimize loss.

We write code to reduce loss in several ways:

1. Loss of time. Programs automate and accelerate actions that would otherwise take people longer to perform.
2. Loss from mistakes. Software should reduce the frequency and impact of errors—both human errors (misclicks, bad inputs, forgotten steps) and system errors (bugs, misconfiguration, unexpected environments). This includes minimizing the cost of failure when mistakes happen.
3. Loss of resources during execution. Programs should run within practical limits: CPU time, memory, disk space, network bandwidth, and monetary cost. Efficiency matters because these resources are finite.
4. Loss of data. Systems should minimize the probability and severity of data loss, through durability mechanisms (backups, redundancy, transactions, checksums, journaling, replication) and careful handling of partial failures.

There is a hierarchy of costs and hard constraints. Every host environment imposes caps: limited memory, disk space, runtime, and deadlines. A program that cannot operate within those constraints is not usable. In that case it does not minimize loss at all—it increases it, because the user spends time attempting to use it and gets no result.

Costs can also be shifted over time. We can accept a one-time setup cost in order to reduce ongoing costs. This is what programming is: we pay the upfront cost of writing software to reduce the recurring cost of executing a process in the future.

A concrete example: a program that reads an entire dataset into memory and processes it all at once may crash if there is not enough memory. For a large enough dataset, this crash is guaranteed. A crash is an extreme form of loss: time is wasted, work may be discarded, and data may be corrupted or left in an inconsistent state. The better design is one that respects constraints—streaming, batching, incremental processing, backpressure, and explicit resource bounds—so the program continues to produce results instead of failing catastrophically.

Some losses are reversible (e.g. a program allocates the memory, then frees it).
Some losses are irreversible (e.g. time)

---

Another important point: don't make up the data.

Examples:

* When parsing a date without timezone: don't assume UTC, return error.
  * Unless the specification explicitly states that this specific date is in UTC timezone.

---

What about the inputs for HTTP requests?

* There is a lot of different inputs
* The user is highly unlikely to want to modify them
* If the request is for a particular API, the user would never want to set any extra parameters, headers, cookies.
