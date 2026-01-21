# Concepts

## Concept

A structure with the following fields:

* Name (required [name](#name) value)
* Definition (required [definition](#definition) value)
* Constructors (optional list of [constructor](#constructor) values that contains all possible constructors of a concept)
* Examples (optional [listmap](#listmap) of [example](#example) values)
* Requirements (optional [listmap](#listmap) of [requirement](#requirement) values)
* Preferences (optional [listmap](#listmap) of [preference](#preference) values)
* Notes (optional [listmap](#listmap) of [note](#note) values)

Examples:

* A definition of a Rust package.
* A definition of a Rust function.

Preferences:

* Should be specific.
* Should contain periods at the ends of the sentences.
  * Reasons:
    * Periods are more common in technical texts than in conversational texts (e.g. reference docs vs forum messages).
    * Periods provide clear boundary cues.

Notes:

* A list of constuctors is exhaustive, while a list of examples is not exhaustive.

## Definition

A single sentence that defines a concept.

Requirements:

* Must end with a period.

Non-requirements:

* May start with an article ("A", "An", "The").
* May start with the name of the concept being defined.

## Constructor

A string that represents a constructor of a type.

## Example

A [stringtree](#stringtree) that [represents](#representation) an instance of a [concept](#concept).

## Requirement

A [stringtree](#stringtree) that [represents](#representation) a boolean test of an instance of a [concept](#concept).

Notes:

* If an input doesn't pass the requirement test, then it is not an instance of a concept.

## Preference

A [stringtree](#stringtree) that [represents](#representation) a less-than-or-equal relation on a pair of instances of a [concept](#concept).

Notes:

* Preferences must be sorted by importance (most important first).
* Preferences should be used to make a choice between two inputs that pass the [requirements](#requirement).

## Note

A [stringtree](#stringtree) that [represents](#representation) additional information about a [concept](#concept).

## Concepts document

A Markdown document that renders a list of [concepts](#concept) with the following elements:

* Heading level 1: "Concepts"
* For each concept:
  * Name: heading level 2.
  * Definition: paragraph after heading.
  * Other fields: paragraph that is exactly equal to the field name with ":" in the end, followed by an unordered list of items for each element in the field value.

Requirements:

* The order of other fields in the document must match the order of other fields in the definition of [concept](#concept).
* A field may not appear twice.
* If the field is empty, it must not be rendered.

## Name

A string that is unique within a document.

## Listmap

A listmap of type A is one of:

* A list of values of type A.
* A map from [names](#name) to values of type A.

Notes:

* If a listmap is a map, its iteration order is the insertion order of keys as they appear in the source text.

## Stringtree

A stringtree is a structure with the following fields:

* Text (string)
* Children (a list of stringtrees)

## Representation

A string that passes an [association test](#association-test) evaluated by a specific list of [agents](#agent).

Notes:

* The list of agents is an external input to a program that evaluates the concept document.

## Association test

A function from two strings to a boolean.

Examples:

* `is("cow", "animal") == true`
* `is("function", "relation") == true`
* `is("war", "peace") == false`
* `is("slavery", "freedom") == false`
* `is("love", "good") == true`

Notes:

* May represent a term-type relation ("cow" has type "animal").
* May represent a subtype-supertype relation ("function" is a subtype of "relation").
* May represent a term-predicate relation ("leaf" is "green").
* System prompt for association testing by LLMs: "Evaluate whether the statement in the user message is true or false in general".

Synonyms:

* is-test

## Agent

An entity whose goal is to prevent its own termination.

Notes:

* An agent can be artificial or natural (LLM or human).
