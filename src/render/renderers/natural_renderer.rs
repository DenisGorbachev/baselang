fn _must_parse_natural_syntax() {
    let _pairs = [
        // "There is A" doesn't read well
        (include_str!("./natural_renderer/something.plain.base"), include_str!("./natural_renderer/something.natural.base")),
    ];
}
