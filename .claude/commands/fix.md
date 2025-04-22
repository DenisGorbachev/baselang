* Read the test file: $ARGUMENTS
* Read other files that are referenced by $ARGUMENTS via `use`
* Find the first test that is marked with `#[ignore]`
* Remove `#[ignore]` from that test
* Execute command: `mise run test:code --nocapture`
* Observe the error
* Read any files that are mentioned in the error
* Debate potential causes of the error
* Fix the issue in the codebase (important: do not modify the test file at $ARGUMENTS, keep the contents of $ARGUMENTS exactly the same)
* Execute command: `mise run test:code --nocapture` until the test is fixed
* Execute command: `mise run fmt` after the test is fixed
