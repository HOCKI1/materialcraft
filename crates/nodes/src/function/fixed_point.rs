// Square a 16-bit fraction, retaining partial products in the top 15 fractional columns.
// A pair of distinct bits contributes twice. A diagonal bit contributes once.
pub(super) fn square_fraction(fraction: u32) -> u32 {
    let mut square = 0;
    for bit in 1..=7 {
        if fraction & (1 << (16 - bit)) != 0 {
            let width = 16 - 2 * bit;
            square += (1 << (width - 1)) + ((fraction >> bit) & ((1 << width) - 1));
        }
    }
    square
}
