#[allow(dead_code)]
pub fn vanilla_binary_search<T: Ord>(sorted: &Vec<T>, target: T) -> Option<usize> {
    let mut left: usize = 0;
    let mut right: usize = sorted.len() - 1;
    while left <= right {
        let mid = (left + right).wrapping_div(2);
        if sorted[mid] == target {
            return Some(mid);
        }
        if sorted[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return None;
}

#[allow(dead_code)]
pub fn find_boundary_binary_search(sorted: &Vec<bool>) -> Option<usize> {
    let mut left: usize = 0;
    let mut right: usize = sorted.len() - 1;
    let mut boundary_index = 0;
    let mut found = false;

    while left <= right {
        let mid = (left + right).wrapping_div(2);
        if sorted[mid] {
            boundary_index = mid;
            found = true;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    return if found { Some(boundary_index) } else { None };
}

pub fn binary_search_first_matching<T: Ord>(input: &Vec<T>, matcher: impl Fn(&T) -> bool) -> Option<usize> {
    let mut left: usize = 0;
    let mut right: usize = input.len() - 1;
    let mut first_matched_index = 0;
    let mut found = false;

    while left <= right {
        let mid = (left + right).wrapping_div(2);
        if matcher(&input[mid]) {
            first_matched_index = mid;
            found = true;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    return if found { Some(first_matched_index) } else { None };
}

fn binary_search_not_smaller<T: Ord>(input: &Vec<T>, target: T) -> Option<usize> {
    binary_search_first_matching(input, |x| *x >= target)
}

#[test]
fn vanilla_binary_search_should_find() {
    let given_vector = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let given_target = 70;
    let result = vanilla_binary_search(&given_vector, given_target);

    assert_eq!(given_vector[result.unwrap()], given_target);
}

#[test]
fn vanilla_binary_search_should_not_find() {
    let given_vector = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let given_target = 25;
    let result = vanilla_binary_search(&given_vector, given_target);

    assert!(result.is_none())
}

#[test]
fn find_boundary_binary_search_should_find() {
    let given_input = vec![false, false, false, true, true, true, true];
    let result = find_boundary_binary_search(&given_input);

    assert_eq!(result.unwrap(), 3);
}

#[test]
fn find_boundary_binary_search_given_only_false_should_not_find() {
    let given_input = vec![false, false, false, false, false];
    let result = find_boundary_binary_search(&given_input);

    assert!(result.is_none());
}

#[test]
fn binary_search_first_matching_should_find() {
    let given_input = vec![false, false, false, true, true, true, true];
    let result = binary_search_first_matching(&given_input, |x| *x == true);

    assert_eq!(result.unwrap(), 3);
}


#[test]
fn binary_search_first_matching_given_only_false_should_not_find() {
    let given_input = vec![false, false, false, false, false];
    let result = binary_search_first_matching(&given_input, |x| *x == true);

    assert!(result.is_none());
}

#[test]
fn binary_search_not_smaller_should_find() {
    let given_input = vec![2, 3, 5, 7, 11, 13, 17, 19];
    let result = binary_search_not_smaller(&given_input, 6);

    assert_eq!(result.unwrap(), 3);
}
