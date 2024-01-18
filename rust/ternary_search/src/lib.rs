use std::cmp::Ordering::{Equal, Greater, Less};

pub fn naive_search<T: PartialEq>(nums: &Vec<T>, target: T) -> Option<usize> {
    for i in 0..nums.len() {
        if nums[i] == target {
            return Some(i);
        }
    }
    return None;
}

// Need to understand Ord vs PartialOrd here as well
pub fn binary_search<T: Ord>(nums: &Vec<T>, target: T) -> Option<usize> {
    let mut l: usize = 0;
    let mut r = nums.len();
    while l < r {
        let mid = l + (r - l) / 2;
        match nums[mid].cmp(&target) {
            Less => {
                l = mid + 1;
            }
            Greater => {
                r = mid;
            }
            Equal => return Some(mid),
        }
    }
    return None;
}

pub fn ternary_search<T: Ord>(nums: &Vec<T>, target: T) -> Option<usize> {
    let mut l: usize = 0;
    let mut r = nums.len();
    while r - l > 2 {
        let offset = (r - l) / 3;
        for i in 1..3 {
            let mid = std::cmp::min(r-1, l + i * offset);
            match nums[mid].cmp(&target) {
                Greater => {
                    r = mid;
                }
                Less => {
                    l = mid + 1;
                }
                Equal => return Some(mid),
            }
        }
    }


    for i in l..r {
        if nums[i] == target {
            return Some(i);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive_search_int() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        assert!(naive_search(&nums, 1) == Some(0));
        assert!(naive_search(&nums, 2) == Some(1));
        assert!(naive_search(&nums, 3) == Some(2));
        assert!(naive_search(&nums, 4) == Some(3));
        assert!(naive_search(&nums, 5) == Some(4));
        assert!(naive_search(&nums, 6) == Some(5));
        assert!(naive_search(&nums, 7) == None);
    }

    #[test]
    fn test_naive_search_str() {
        let nums = vec!["a", "b", "c", "d", "e", "f"];
        assert!(naive_search(&nums, "a") == Some(0));
        assert!(naive_search(&nums, "b") == Some(1));
        assert!(naive_search(&nums, "c") == Some(2));
        assert!(naive_search(&nums, "d") == Some(3));
        assert!(naive_search(&nums, "e") == Some(4));
        assert!(naive_search(&nums, "f") == Some(5));
        assert!(naive_search(&nums, "g") == None);
    }

    #[test]
    fn test_binary_search_int() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        assert!(binary_search(&nums, 1) == Some(0));
        assert!(binary_search(&nums, 2) == Some(1));
        assert!(binary_search(&nums, 3) == Some(2));
        assert!(binary_search(&nums, 4) == Some(3));
        assert!(binary_search(&nums, 5) == Some(4));
        assert!(binary_search(&nums, 6) == Some(5));
        assert!(binary_search(&nums, 7) == None);
    }

    #[test]
    fn test_binary_search_str() {
        let nums = vec!["a", "b", "c", "d", "e", "f"];
        assert!(binary_search(&nums, "a") == Some(0));
        assert!(binary_search(&nums, "b") == Some(1));
        assert!(binary_search(&nums, "c") == Some(2));
        assert!(binary_search(&nums, "d") == Some(3));
        assert!(binary_search(&nums, "e") == Some(4));
        assert!(binary_search(&nums, "f") == Some(5));
        assert!(binary_search(&nums, "g") == None);
    }

    #[test]
    fn test_ternary_search_int() {
        let nums = vec![1, 2, 3, 4, 8, 10];
        assert!(ternary_search(&nums, 1) == Some(0));
        assert!(ternary_search(&nums, 2) == Some(1));
        assert!(ternary_search(&nums, 3) == Some(2));
        assert!(ternary_search(&nums, 4) == Some(3));
        assert!(ternary_search(&nums, 8) == Some(4));
        assert!(ternary_search(&nums, 10) == Some(5));
        assert!(ternary_search(&nums, 7) == None);
    }
}
