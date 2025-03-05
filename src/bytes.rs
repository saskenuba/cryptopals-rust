/// Splits the slice into multiple slices of `size`, and joins each byte of the new slices into a
/// separate vector of the corresponding index.
///
/// Sample:
/// ```
/// size = 2;
/// block = [10, 20, 30, 40];
///
/// vec[0] = [10, 30];
/// vec[1] = [20, 40];
/// ```
pub fn transpose_bytes_block(slice: &[u8], size: i32) -> Vec<Vec<u8>> {
    let mut buffer: Vec<Vec<_>> = (0..size)
        .map(|_| Vec::with_capacity(slice.len() / size as usize))
        .collect();

    for (idx, &byte) in slice.iter().enumerate() {
        let idx = idx % size as usize;
        let idx_vec = unsafe { buffer.get_unchecked_mut(idx) };

        idx_vec.push(byte)
    }
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_transpose() {
        let source = [10, 20, 30, 40, 50];
        let res = transpose_bytes_block(source.as_slice(), 2);
        assert_eq!(res[0], &[10, 30, 50]);
        assert_eq!(res[1], &[20, 40]);
        assert_eq!(res.get(2), None);
    }
}
