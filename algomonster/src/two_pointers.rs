/// Removes duplicate elements from a sorted vector in-place
/// and returns the number of unique elements.
///
/// This function modifies the given vector such that the first
/// portion contains all unique elements in sorted order without duplicates, 
/// and the remaining portion of the vector is left undefined.
///
/// # Parameters
/// - `arr`: A mutable reference to a vector of i32 values. 
///   The vector must be sorted prior to calling this function, 
///   as the function assumes the sorted order of elements for removing duplicates.
///
/// # Returns
/// - The number of unique elements in the vector after removing duplicates. 
///
/// # Behavior
/// - If the input vector `arr` is empty, the function immediately returns 0.
/// - The order of the unique elements is preserved.
/// - The vector is modified in place, overwriting duplicate values with unique ones.
///
/// # Complexity  
/// - Time Complexity: O(n), where `n` is the length of the vector. Each element is visited once.
/// - Space Complexity: O(1), as the operation is performed in-place.
///
/// # Examples
/// ```
/// let mut vec = vec![1, 1, 2, 2, 3, 4, 4];
/// let unique_count = remove_duplicates(&mut vec);
/// assert_eq!(unique_count, 4);
/// assert_eq!(&vec[..unique_count], &[1, 2, 3, 4]);
///
/// let mut empty_vec: Vec<i32> = vec![];
/// assert_eq!(remove_duplicates(&mut empty_vec), 0);
/// assert_eq!(empty_vec, vec![]);
/// ```
///
/// # Debugging
/// - A `debug_assert!` ensures that the `write_index` never exceeds the current iteration index `i`.
///
/// # Notes
/// Ensure that the input vector is sorted before calling this function. 
/// If the input vector is not sorted, the behavior of the function is undefined.
fn remove_duplicates(arr: &mut Vec<i32>) -> usize {
    if arr.is_empty() {
        return 0;
    }
    let mut write_index = 1;

    for i in 1..arr.len() {
        debug_assert!(write_index <= i);
        if arr[i] != arr[i - 1] {
            arr[write_index] = arr[i];
            write_index += 1;
        }
    }
    write_index
}
type List<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: List<T>,
}


/// Finds and returns the value of the middle node in a singly linked list.
///
/// This function uses the two-pointer technique with `slow` and `fast` pointers to determine
/// the middle of the linked list in O(n) time complexity. The `fast` pointer advances twice
/// as fast as the `slow` pointer. When the `fast` pointer reaches the end of the list,
/// the `slow` pointer will be at the middle.
///
/// # Arguments
/// * `head` - The head of the singly linked list is represented as `List<i32>`.
///
/// # Returns
/// * `i32` - The value of the middle node in the singly linked list. If the list has an even 
///   number of nodes, the function returns the value of the second middle node.
///
/// # Example
/// ```
/// // Input: Linked list with nodes [1, 2, 3, 4, 5]
/// let result = middle_of_linked_list(head);
/// assert_eq!(result, 3); // Middle of the linked list
/// ```
///
/// # Assumptions
/// * The input list is non-empty.
/// * The list structure is implemented correctly and provides access to `.next`
///   and `.val` fields.
///
/// # Panics
/// * If the linked list is empty, the function will panic due to an unwrapping of a `None` value.
///
/// # Complexity
/// * Time Complexity: O(n), where n is the number of nodes in the linked list.
/// * Space Complexity: O(1), as no additional data structures are used.
fn middle_of_linked_list(head: List<i32>) -> i32 {
    let mut slow = &head;
    let mut fast = &head;

    while let Some(curr) = fast {
        if let Some(next) = &curr.next {
            fast = &next.next;
            slow = &slow.as_ref().unwrap().next;
        } else {
            break;
        }
    }

    slow.as_ref().unwrap().val
}

/// Finds the indices of two numbers in a sorted vector that add up to a given target.
///
/// This function assumes the input vector `arr` is sorted in ascending order.
/// It uses a two-pointer approach to find the indices of the two numbers that 
/// add up to the specified target. If such a pair is found, their indices are 
/// returned in a vector. Otherwise, an empty vector is returned.
///
/// # Arguments
///
/// * `arr` - A `Vec<i32>` representing a sorted input array.
/// * `target` - An `i32` representing the target sum to find.
///
/// # Returns
///
/// A `Vec<i32>` containing the indices of the two elements whose sum equals
/// the target. If no such pair exists, an empty vector is returned. The indices
/// in the result correspond to the positions of the elements in the original
/// input vector.
///
/// # Examples
///
/// ```
/// let arr = vec![1, 2, 3, 4, 6];
/// let target = 7;
/// let result = two_sum_sorted(arr, target);
/// assert_eq!(result, vec![1, 3]); // 2 + 4 = 7
///
/// let arr = vec![1, 3, 5, 7];
/// let target = 10;
/// let result = two_sum_sorted(arr, target);
/// assert_eq!(result, vec![1, 2]); // 3 + 5 = 10
///
/// let arr = vec![2, 4, 6];
/// let target = 10;
/// let result = two_sum_sorted(arr, target);
/// assert!(result.is_empty()); // No pair adds up to 10
/// ```
///
/// # Complexity
///
/// - Time Complexity: O(n), where `n` is the length of the input vector. This is 
///   because each iteration of the while loop processes one element from either 
///   end of the vector.
/// - Space Complexity: O(1), as no additional space is used apart from the result 
///   vector.
fn two_sum_sorted(arr: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut result = Vec::new();
    
    while left < right {
        let sum = arr[left] + arr[right];
        if sum == target {
            result.push(left as i32);
            result.push(right as i32);
            break;
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    result
}

/// Calculates the maximum area of water that can be contained by two vertical lines on a Cartesian plane.
///
/// This function finds the maximum area that can be formed between two lines
/// selected from the given heights, where the `height` vector represents the vertical
/// heights of the lines occurring at respective indices. The horizontal distance
/// between two chosen lines is the difference in their indices, and the area is determined
/// by the shorter of the two heights multiplied by their distance.
///
/// # Parameters
/// - `height`: A vector of integers where each value represents the height of a line.
///
/// # Returns
/// - An integer representing the maximum area of water that two selected vertical lines can contain.
///
/// # Algorithm
/// 1. The function uses two pointers, `left` (starting at the beginning) and `right` (starting at the end),
///    to iterate through the vector of heights.
/// 2. The area between the two pointers is calculated using the formula:
///    `area = (right - left) * min(height[left], height[right])`.
/// 3. The `max_area` variable keeps track of the largest area calculated so far.
/// 4. Depending on the height of the lines at the `left` and `right` pointers, the pointer at the smaller
///    height is moved inward to maximize potential area.
/// 5. This process continues until the two pointers meet.
///
/// # Complexity
/// - Time Complexity: O(n)
///   The two-pointer approach ensures that each index is processed at most once.
/// - Space Complexity: O(1)
///   Only a constant amount of extra space is used.
///
/// # Example
/// ```
/// let heights = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
/// let result = container_with_max_area(heights);
/// assert_eq!(result, 49); // The maximum area is formed between heights at indices 1 and 8.
/// ```
fn container_with_max_area(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        return 0;
    }
    let mut max_area = 0;
    let mut left = 0;
    let mut right = height.len() - 1 ;
    while left < right {
        let area = (right - left) as i32 * std::cmp::min(height[left], height[right]);
        max_area = std::cmp::max(max_area, area);
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;       
        }
    }
    max_area   
}
#[cfg(test)]
mod test {
    use crate::two_pointers::{remove_duplicates, middle_of_linked_list, List, Node, two_sum_sorted, container_with_max_area};

    // Helper function to create a linked list from a vector for easier testing
    fn to_list(vec: Vec<i32>) -> List<i32> {
        let mut current_list = None;
        for &val in vec.iter().rev() {
            current_list = Some(Box::new(Node {
                val,
                next: current_list,
            }));
        }
        current_list
    }

    #[test]
    fn remove_duplicates_with_duplicats_returns_new_size() {
        let mut arr: Vec<i32> = vec![1, 1, 1, 2, 2, 3, 3, 3];

        let size_after_removed_duplicates = remove_duplicates(&mut arr);
        assert_eq!(&arr[..size_after_removed_duplicates], &[1, 2, 3]);
    }

    #[test]
    fn remove_duplicates_without_duplicates_returns_new_size() {
        let mut arr: Vec<i32> = vec![1, 2, 3];

        let size_after_removed_duplicates = remove_duplicates(&mut arr);
        assert_eq!(&arr[..size_after_removed_duplicates], &[1, 2, 3]);
    }

    #[test]
    fn remove_duplicates_empty_input_returns_zero() {
        let mut arr: Vec<i32> = Vec::new();

        let size_after_removed_duplicates = remove_duplicates(&mut arr);
        assert_eq!(&arr[..size_after_removed_duplicates], &[]);
    }

    #[test]
    fn middle_of_linked_list_odd_length() {
        let list = to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(middle_of_linked_list(list), 3);
    }

    #[test]
    fn middle_of_linked_list_even_length() {
        let list = to_list(vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(middle_of_linked_list(list), 4);
    }

    #[test]
    fn middle_of_linked_list_single_node() {
        let list = to_list(vec![1]);
        assert_eq!(middle_of_linked_list(list), 1);
    }

    #[test]
    #[should_panic]
    fn middle_of_linked_list_empty_list_panics() {
        let list: List<i32> = None;
        middle_of_linked_list(list);
    }

    #[test]
    fn middle_of_linked_list_two_nodes() {
        let list = to_list(vec![1, 2]);
        assert_eq!(middle_of_linked_list(list), 2);
    }

    #[test]
    fn two_sum_sorted_finds_target() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(two_sum_sorted(arr, 7), vec![1, 4]);
    }

    #[test]
    fn two_sum_sorted_target_at_ends() {
        let arr = vec![1, 3, 5, 7, 9];
        assert_eq!(two_sum_sorted(arr, 10), vec![0, 4]);
    }

    #[test]
    fn two_sum_sorted_no_solution() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(two_sum_sorted(arr, 10), vec![]);
    }

    #[test]
    fn two_sum_sorted_with_duplicates() {
        let arr = vec![1, 2, 2, 3, 4];
        assert_eq!(two_sum_sorted(arr, 4), vec![0, 3]);
    }

    #[test]
    fn container_with_max_area_typical_case() {
        let heights = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(container_with_max_area(heights), 49);
    }

    #[test]
    fn container_with_max_area_empty() {
        let heights = vec![];
        assert_eq!(container_with_max_area(heights), 0);
    }

    #[test]
    fn container_with_max_area_two_elements() {
        let heights = vec![1, 2];
        assert_eq!(container_with_max_area(heights), 1);
    }

    #[test]
    fn container_with_max_area_non_adjacent() {
        let heights = vec![1, 2, 4, 3, 7];
        assert_eq!(container_with_max_area(heights), 8);
    }
}