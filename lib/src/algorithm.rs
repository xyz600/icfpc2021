pub fn next_permutation(vec: &mut Vec<usize>) -> bool {
    for i in (0..vec.len() - 1).rev() {
        if vec[i] < vec[i + 1] {
            for j in (i + 1..vec.len()).rev() {
                if vec[i] < vec[j] {
                    vec.swap(i, j);
                    vec[i + 1..].reverse();
                    return true;
                }
            }
        }
    }
    false
}

#[test]
fn test_next_permutation() {
    let mut vec = vec![0, 1, 2, 3];
    assert!(next_permutation(&mut vec));
    assert_eq!(vec![0, 1, 3, 2], vec);
    assert!(next_permutation(&mut vec));
    assert_eq!(vec![0, 2, 1, 3], vec);
    assert!(next_permutation(&mut vec));
    assert_eq!(vec![0, 2, 3, 1], vec);
    assert!(next_permutation(&mut vec));
    assert_eq!(vec![0, 3, 1, 2], vec);
    assert!(next_permutation(&mut vec));
    assert_eq!(vec![0, 3, 2, 1], vec);
}

pub fn next_duplicated_permutation(state: &mut Vec<usize>, max_value: usize) -> bool {
    for i in 0..state.len() {
        if state[i] != max_value {
            state[i] += 1;
            for j in 0..i {
                state[j] = 0;
            }
            return true;
        }
    }
    false
}

#[test]
fn test_next_duplicated_permutation() {
    let mut vec = vec![0, 0, 0, 0];
    assert!(next_duplicated_permutation(&mut vec, 2));
    assert_eq!(vec![0, 0, 0, 1], vec);
    assert!(next_duplicated_permutation(&mut vec, 2));
    assert_eq!(vec![0, 0, 0, 2], vec);
    assert!(next_duplicated_permutation(&mut vec, 2));
    assert_eq!(vec![0, 0, 1, 0], vec);
}
