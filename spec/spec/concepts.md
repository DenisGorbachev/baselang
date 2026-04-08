# Spec of spec

## Metrics

- Code size
- Prompt length in tokens
- Probability of the agent implementing the spec from the prompt
  - Probability of a one-shot fix ("get errors, notice errors is not empty, ask agent to fix, get errors, notice errors is empty")

## Concepts

### fn main

- Must write the errors to stdout
  - Should write the errors to stdout as soon as they are discovered
    - Notes:
      - This may conflict with the "Code size" metric, which has a higher priority

### TestReport struct

A struct whose every field is either [Outcome](#outcome-enum) or another TestReport struct.

### Outcome enum

```rust
#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Outcome {
    Pass,
    #[default]
    Fail,
}
```
