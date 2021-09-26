pub fn vec_find_subslice<T: PartialEq>(haystack: &[T], needle: &[T]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }

    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

pub fn vec_contains<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    vec_find_subslice(haystack, needle).is_some()
}

pub fn vec_window_split<'a, T: PartialEq>(vec: &'a [T], separator: &[T]) -> Vec<&'a [T]> {
    if separator.is_empty() {
        if vec.is_empty() {
            return vec![vec];
        } else {
            return vec.windows(1).collect();
        }
    }

    let mut ret: Vec<&'a [T]> = vec![];
    let mut prev_index = 0;
    // Skip entries to advanced the iterator after a match is found
    let mut skip_entries = 0;
    for (cur_index, window) in vec.windows(separator.len()).enumerate() {
        if skip_entries > 0 {
            skip_entries -= 1;
        } else if window == separator {
            ret.push(&vec[prev_index..cur_index]);
            prev_index = cur_index + separator.len();
            skip_entries = separator.len() - 1;
        }
    }
    ret.push(&vec[prev_index..]);

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_find_subslice() {
        assert_eq!(vec_find_subslice::<u8>(&[], &[]), Some(0));
        assert_eq!(vec_find_subslice(&[1], &[1]), Some(0));

        assert_eq!(vec_find_subslice(&[1, 2, 3], &[1]), Some(0));
        assert_eq!(vec_find_subslice(&[1, 2, 3], &[2]), Some(1));
        assert_eq!(vec_find_subslice(&[1, 2, 3], &[3]), Some(2));

        assert_eq!(vec_find_subslice(&[1, 2, 3], &[1, 2]), Some(0));
        assert_eq!(vec_find_subslice(&[1, 2, 3], &[2, 3]), Some(1));

        assert_eq!(vec_find_subslice(&[1, 2, 3], &[1, 2, 3]), Some(0));

        assert_eq!(vec_find_subslice(&[], &[1]), None);
        assert_eq!(vec_find_subslice(&[1], &[2]), None);
        assert_eq!(vec_find_subslice(&[1, 2, 3], &[1, 3]), None);
        assert_eq!(vec_find_subslice(&[1, 2, 3], &[1, 2, 3, 4]), None);
    }

    #[test]
    fn vec_contains_true() {
        assert!(vec_contains::<u8>(&[], &[]));
        assert!(vec_contains(&[1], &[1]));

        assert!(vec_contains(&[1, 2, 3], &[1]));
        assert!(vec_contains(&[1, 2, 3], &[2]));
        assert!(vec_contains(&[1, 2, 3], &[3]));

        assert!(vec_contains(&[1, 2, 3], &[1, 2]));
        assert!(vec_contains(&[1, 2, 3], &[2, 3]));

        assert!(vec_contains(&[1, 2, 3], &[1, 2, 3]));
    }

    #[test]
    fn vec_contains_false() {
        assert!(!vec_contains(&[], &[1]));
        assert!(!vec_contains(&[1], &[2]));
        assert!(!vec_contains(&[1, 2, 3], &[1, 3]));
        assert!(!vec_contains(&[1, 2, 3], &[1, 2, 3, 4]));
    }

    #[test]
    fn test_vec_window_split() {
        assert_eq!(v1(&[]), vec_window_split(&[], &[]));
        assert_eq!(v1(&[]), vec_window_split(&[], &[1]));

        assert_eq!(v2(&[], &[]), vec_window_split(&[1], &[1]));
        assert_eq!(v2(&[0], &[2]), vec_window_split(&[0, 1, 2], &[1]));
        assert_eq!(
            v3(&[1], &[], &[1]),
            vec_window_split(&[1, 8, 9, 8, 9, 1], &[8, 9])
        );
        assert_eq!(
            v2(&[0, 1, 2], &[0]),
            vec_window_split(&[0, 1, 2, 1, 1, 0], &[1, 1])
        );
        assert_eq!(
            v4(&[0], &[0], &[0], &[]),
            vec_window_split(&[0, 1, 0, 1, 0, 1], &[1])
        );
        assert_eq!(v2(&[], &[1]), vec_window_split(&[1, 1, 1], &[1, 1]));
        assert_eq!(v3(&[1], &[1], &[1]), vec_window_split(&[1, 1, 1], &[]));
    }

    // These are helper functions for building vectors.

    fn v1(a: &[i32]) -> Vec<&[i32]> {
        vec![a]
    }

    fn v2<'a>(a: &'a [i32], b: &'a [i32]) -> Vec<&'a [i32]> {
        vec![a, b]
    }

    fn v3<'a>(a: &'a [i32], b: &'a [i32], c: &'a [i32]) -> Vec<&'a [i32]> {
        vec![a, b, c]
    }

    fn v4<'a>(a: &'a [i32], b: &'a [i32], c: &'a [i32], d: &'a [i32]) -> Vec<&'a [i32]> {
        vec![a, b, c, d]
    }
}
