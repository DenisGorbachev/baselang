# Baselang

## Associations

The basic unit of thought is "association" - a link between two concepts. Let's use `~` to denote a simple association. For example:

```text
Sky ~ Blue
Water ~ Life
Food ~ Good
```

These associations don't hold in general, but do hold sometimes.

Let's try expressing the following observation: Alice got sick, then she ate red berries, then she got healthy.

Our first attempt could be:

```text
Alice ~ Sick
Alice ~ Berries
Berries ~ Red
Alice ~ Healthy
```

However, this text has multiple issues:

- It doesn't express the timing (e.g. that Alice got healthy _after_ eating berries).
