pub fn frand(seed: u32) -> impl Iterator<Item = f64> {
    let mut r = seed;

    std::iter::repeat_with(move || {
        r ^= r << 13;
        r ^= r >> 17;
        r ^= r << 5;
        r
    })
    .map(|value| (value as f64) / (std::u32::MAX as f64))
}
