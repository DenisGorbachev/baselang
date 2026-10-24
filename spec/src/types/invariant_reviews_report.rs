use crate::{CheckRequirementError, CheckTypeInvariantsError, report};

report! {
    pub struct InvariantReviewsReport {
        pub app: Result<(), CheckTypeInvariantsError>,
        pub sym: Result<(), CheckTypeInvariantsError>,
        pub rewrite: Result<(), CheckTypeInvariantsError>,
        pub scope: Result<(), CheckTypeInvariantsError>,
        pub theory: Result<(), CheckTypeInvariantsError>,
        pub typ: Result<(), CheckTypeInvariantsError>,
        pub exp: Result<(), CheckTypeInvariantsError>,
        pub duo_vec: Result<(), CheckTypeInvariantsError>,
        pub apply: Result<(), CheckRequirementError>,
    }
}
