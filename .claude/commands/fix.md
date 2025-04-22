* Read the test file: $ARGUMENTS
* Read other files that are referenced by $ARGUMENTS via `use`
* Find the first test that is marked with `#[ignore]`
* Remove `#[ignore]` from that test
* Execute command: `mise run test:code --nocapture`
* Observe the error
* Read any files that are mentioned in the error
* Debate potential causes of the error
* Apply a fix (important: do not modify the test itself)
* Execute command: `mise run test:code --nocapture` until the test is fixed
* Execute command: `mise run fmt` after the test is fixed
