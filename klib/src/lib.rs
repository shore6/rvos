#![cfg_attr(not(test), no_std)]

pub const fn align_up(value: usize, align: usize) -> usize {
    debug_assert!(align.is_power_of_two(), "align must be a power of two.");
    (value + align - 1) & !(align - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_up() {
        assert_eq!(align_up(3, 1), 3);
        assert_eq!(align_up(3, 2), 4);
        assert_eq!(align_up(5, 4), 8);
        assert_eq!(align_up(22, 8), 24);
        assert_eq!(align_up(31, 16), 32);
        assert_eq!(align_up(8, 8), 8);
        assert_eq!(align_up(0, 4), 0);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "align must be a power of two.")]
    fn test_align_is_odd() {
        let _ = align_up(4, 3);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "align must be a power of two.")]
    fn test_align_is_not_power_of_two() {
        let _ = align_up(4, 6);
    }
}
