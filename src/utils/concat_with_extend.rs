pub fn concat_with_extend<T>(vecs: Vec<Vec<T>>) -> Vec<T> {
    // 1. Compute total length N
    let total: usize = vecs.iter().map(Vec::len).sum();

    // 2. Allocate once: capacity = N
    let mut out = Vec::with_capacity(total);

    // 3. Move each vec’s contents into `out`
    for mut v in vecs {
        // extend() takes ownership of `v`’s elements (no clone)
        out.append(&mut v);
        // after drain, v.capacity drops out of scope
    }

    out
}
