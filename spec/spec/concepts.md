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
    - Must maintain the context for errors
      - #notes
        - The standard context preservation technique (returning owned vars while bubbling up the error) will not work for streamed errors
          - #notes
            - Context contains the vars from callers
            - Callee doesn't have access to that context
          - #options
            - Print the errors at the top level
            - Pass an ErrorContext into callees
              - Require to register vars as Dyn Debug
                - But that would require holding a ref
            - Do not require to stream errors, allow reporting them collectively
    - #notes
      - This preference may conflict with the "Code size" metric, which has a higher priority

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
