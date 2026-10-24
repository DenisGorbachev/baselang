use crate::CheckOutcome;
use std::error::Error;
use std::iter::once;

pub trait Report {
    fn outcomes(&self) -> impl Iterator<Item = CheckOutcome<'_>>;

    fn passed(&self) -> bool;
}

impl Report for () {
    fn passed(&self) -> bool {
        true
    }

    fn outcomes(&self) -> impl Iterator<Item = CheckOutcome<'_>> {
        once(CheckOutcome {
            name: String::new(),
            result: Ok(()),
        })
    }
}

impl<T: Report, E: Error> Report for Result<T, E> {
    fn passed(&self) -> bool {
        matches!(self, Ok(value) if value.passed())
    }

    fn outcomes(&self) -> impl Iterator<Item = CheckOutcome<'_>> {
        let (value, error) = match self {
            Ok(value) => (Some(value), None),
            Err(error) => (
                None,
                Some(CheckOutcome {
                    name: String::new(),
                    result: Err(error),
                }),
            ),
        };
        value.into_iter().flat_map(Report::outcomes).chain(error)
    }
}
