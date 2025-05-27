use std::collections::HashMap;

/// Removes duplicate elements from a sorted vector in-place
/// and returns the number of unique elements.
///
/// This function modifies the given vector such that the first
/// portion contains all unique elements in sorted order without duplicates,
/// and the remaining portion of the vector is left undefined.
///
/// # Parameters
/// - `arr`: A mutable reference to a vector of i32 values.
///   The vector must be sorted before calling this function,
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
#[allow(dead_code)]
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

#[derive(PartialEq)]
struct Node<T> {
    val: T,
    next: List<T>,
}

#[allow(dead_code)]
fn has_cycle(head: List<i32>) -> bool {
    let mut slow = &head;
    let mut fast = &head;
    while let Some(curr) = fast {
        if let Some(next) = &curr.next {
            fast = &next.next;
            slow = &slow.as_ref().unwrap().next;
        } else {
            break;
        }
        if slow == fast {
            return true;
        }
    }
    false
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

#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
fn container_with_max_area(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        return 0;
    }
    let mut max_area = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
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
/// Finds the maximum sum of a fixed-length subarray within a given array.
///
/// This function uses a sliding window approach to find the subarray of length `k`
/// with the maximum sum in the input array.
///
/// # Arguments
/// * `arr` - A vector of integers representing the input array
/// * `k` - The fixed length of the subarray to consider
///
/// # Returns
/// * `i32` - The maximum sum found among all possible subarrays of length `k`
///
/// # Example
/// ```
/// let arr = vec![1, 4, 2, 10, 2, 3, 1, 0, 20];
/// let k = 4;
/// let max_sum = subarray_sum_fixed_length(arr, k);
/// assert_eq!(max_sum, 24); // sum of subarray [2, 10, 2, 3]
/// ```
///
/// # Complexity
/// * Time Complexity: O(n), where n is the length of the input array
/// * Space Complexity: O(1)

#[allow(dead_code)]
fn subarray_sum_fixed_length(arr: Vec<i32>, k: usize) -> i32 {
    let mut window_sum = 0;
    for i in 0..k {
        window_sum += arr[i];
    }

    let mut largest = window_sum;
    for right in k..arr.len() {
        window_sum += arr[right];
        window_sum -= arr[right - k];
        largest = std::cmp::max(largest, window_sum);
    }
    largest
}

/// Finds a contiguous subarray within a given array `arr` that sums up to a specified target value
/// and returns the 1-based starting and ending indices of that subarray. If no such subarray exists,
/// it returns an empty vector.
///
/// # Arguments
///
/// * `arr` - A vector of integers, representing the array to search for the subarray.
/// * `target` - An integer, representing the target sum to look for in the subarray.
///
/// # Returns
///
/// * A vector of two integers containing the 1-based starting and ending indices of the subarray
///   that sums to the target value. Returns an empty vector if such a subarray cannot be found.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
///
/// let arr = vec![1, 2, 3, 4, 5];
/// let target = 9;
/// let result = subarray_sum(arr, target);
/// assert_eq!(result, vec![2, 4]); // Subarray [2, 3, 4] sums to 9.
///
/// let arr = vec![1, -1, 5, -2, 3];
/// let target = 3;
/// let result = subarray_sum(arr, target);
/// assert_eq!(result, vec![1, 4]); // Subarray [1, -1, 5, -2] sums to 3.
///
/// let arr = vec![1, 2, 3];
/// let target = 10;
/// let result = subarray_sum(arr, target);
/// assert_eq!(result, Vec::<i32>::new()); // No subarray sums to 10.
/// ```
///
/// # Notes
///
/// The function uses a prefix sum approach with a `HashMap` to track cumulative sums encountered
/// during iteration. The key idea is to compute the complement of the current cumulative sum
/// relative to the target and check whether it exists in the hash map. If found, a valid subarray
/// exists starting from the next position of the complement to the current position (both inclusive).
///
/// The function assumes the input array may contain positive, negative, or zero integers.

#[allow(dead_code)]
fn subarray_sum(arr: Vec<i32>, target: i32) -> Vec<i32> {
    let mut prefix_sums: HashMap<i32, i32> = HashMap::new();
    prefix_sums.insert(0, 0);
    let mut sum = 0;
    for i in 0..arr.len() {
        sum += arr[i];
        let complement = sum - target;
        if let Some(prefix_sum_end) = prefix_sums.get(&complement) {
            return vec![prefix_sum_end.clone(), (i + 1) as i32];
        }
        let _ = *prefix_sums.entry(sum).or_insert((i + 1) as i32);
    }
    Vec::new()
}

/// Calculates the total number of continuous subarrays within a given array whose sums equal a specific target value.
///
/// This function uses a prefix sum approach with a hash map to efficiently calculate the number of subarrays that satisfy the condition in O(n) time complexity.
///
/// # Arguments
///
/// * `arr` - A vector of integers representing the input array.
/// * `target` - An integer representing the target sum for the subarray.
///
/// # Returns
///
/// * An integer representing the total number of continuous subarrays within `arr` whose sums equal `target`.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
///
/// let arr = vec![1, 1, 1];
/// let target = 2;
/// let result = subarray_sum_total(arr, target);
/// assert_eq!(result, 2); // There are two subarrays: [1, 1] and [1, 1].
/// ```
///
/// # Algorithm
///
/// 1. Initialize a hash map (`prefix_sums`) to store prefix sums and their frequencies. Insert an entry for prefix sum 0 with a count of 1.
/// 2. Use a running sum (`curr_sum`) to calculate the cumulative sum of the array as you iterate through it.
/// 3. For each element in the array, calculate the complement as `curr_sum - target`.
/// 4. Check if the complement exists in the hash map (`prefix_sums`). If it does, add its frequency to the result count (`count`).
/// 5. Update the frequency of the current prefix sum in the hash map.
/// 6. Return the total count of subarrays that satisfy the condition.
///
/// # Note
///
/// This implementation requires the `HashMap` from the standard library. Ensure it is imported before calling the function.

#[allow(dead_code)]
fn subarray_sum_total(arr: Vec<i32>, target: i32) -> i32 {
    let mut prefix_sums: HashMap<i32, i32> = HashMap::new();
    prefix_sums.insert(0, 1);
    let mut curr_sum = 0;
    let mut count = 0;

    for val in arr {
        curr_sum += val;
        let complement = curr_sum - target;
        if let Some(prefix_sum_end) = prefix_sums.get(&complement) {
            count += prefix_sum_end;
        }
        *prefix_sums.entry(curr_sum).or_insert(0) += 1;
    }
    count
}

/// Computes the sum of elements in the given range `[left, right]` (inclusive) for an immutable array of integers.
///
/// This function uses a prefix sum approach to perform efficient range sum queries.
/// By precomputing the cumulative sums for the array, it allows for constant-time range sum queries after
/// an O(n) preprocessing step. The prefix sums are stored in an auxiliary array (`prefix_sums`) where
/// each element represents the sum of all elements in the input array up to that index.
///
/// # Arguments
///
/// * `nums` - A vector of integers, representing the input array.
/// * `left` - A `usize` value indicating the starting index of the range (inclusive).
/// * `right` - A `usize` value indicating the ending index of the range (inclusive).
///
/// # Returns
///
/// * An `i32` value representing the sum of elements between indices `left` and `right` (inclusive) in the original array.
///
/// # Complexity
///
/// * Time Complexity:
///   * Precomputing prefix sums: O(n), where `n` is the length of `nums`.
///   * Querying the range sum: O(1).
/// * Space Complexity: O(n) for storing the prefix sums.
///
/// # Example
///
/// ```
/// let nums = vec![1, 2, 3, 4, 5];
/// let left = 1;
/// let right = 3;
/// let sum = range_sum_query_immutable(nums, left, right);
/// assert_eq!(sum, 9); // Sum of nums[1..=3] = 2 + 3 + 4 = 9
/// ```
///
/// # Edge Cases
///
/// * If the `left` and `right` indices are out of bounds of the array, this function may panic.
/// * If `left` == `right`, the function returns the single element at that index.
/// * If the input vector is empty, the behavior will be undefined or may panic.
///

#[allow(dead_code)]
fn range_sum_query_immutable(nums: Vec<i32>, left: usize, right: usize) -> i32 {
    let mut prefix_sums: Vec<i32> = vec![0; nums.len() + 1];
    for i in 0..nums.len() {
        prefix_sums[i + 1] = prefix_sums[i] + nums[i];
    }
    prefix_sums[right + 1] - prefix_sums[left]
}

/// Computes the product of all elements in the array except the current index for each element.
///
/// This function takes a slice of integers as input and returns a vector where each
/// element at index `i` is the product of all the elements in the input array except
/// the one at index `i`. It achieves this without using division by precomputing the
/// prefix and suffix products for each element.
///
/// # Arguments
/// * `nums` - A slice of integers for which the product of elements excluding the current one is computed.
///
/// # Returns
/// A vector of integers where each element at index `i` is the product of all the elements
/// from the given array except the one at index `i`.
///
/// # Edge Cases
/// * If the input array is empty (`nums` is empty), the function will return an
///   empty vector.
/// * The function assumes all elements in `nums` are valid integers and doesn't
///   handle potential overflow cases for very large products.
///
/// # Time Complexity
/// * O(n): The function iterates through the array twice, first to compute the prefix
///   products and then the suffix products, resulting in linear time complexity.
///
/// # Space Complexity
/// * O(n): The function uses an additional vector of the same size as the input
///   array to store results.
///
/// # Examples
///
/// ```
/// let nums = vec![1, 2, 3, 4];
/// let result = product_of_array_except_self(&nums);
/// assert_eq!(result, vec![24, 12, 8, 6]);
/// ```
///
/// ```
/// let nums = vec![0, 1, 2, 3];
/// let result = product_of_array_except_self(&nums);
/// assert_eq!(result, vec![6, 0, 0, 0]);
/// ```
///
/// ```
/// let nums: Vec<i32> = vec![];
/// let result = product_of_array_except_self(&nums);
/// assert_eq!(result, vec![]);
/// ```

#[allow(dead_code)]
fn product_of_array_except_self(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();

    if n == 0 {
        return Vec::new();
    }

    let mut result = vec![1; n];

    let mut prefix_product = 1;
    for i in 0..n {
        result[i] = prefix_product;
        prefix_product *= nums[i];
    }

    let mut suffix_product = 1;
    for i in (0..n).rev() {
        result[i] *= suffix_product;
        suffix_product *= nums[i];
    }

    result
}

#[cfg(test)]
mod test {
    use crate::two_pointers::{
        container_with_max_area, middle_of_linked_list, product_of_array_except_self,
        range_sum_query_immutable, remove_duplicates, subarray_sum, subarray_sum_fixed_length,
        subarray_sum_total, two_sum_sorted, List, Node,
    };

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
    fn remove_duplicates_with_duplicates_returns_new_size() {
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

    #[test]
    fn subarray_sum_fixed_length_typical_case() {
        let arr = vec![1, 4, 2, 10, 2, 3, 1, 0, 20];
        assert_eq!(subarray_sum_fixed_length(arr, 4), 24);
    }

    #[test]
    fn subarray_sum_fixed_length_entire_array() {
        let arr = vec![1, 2, 3];
        assert_eq!(subarray_sum_fixed_length(arr, 3), 6);
    }

    #[test]
    fn subarray_sum_fixed_length_single_element() {
        let arr = vec![5, 2, 1, 3];
        assert_eq!(subarray_sum_fixed_length(arr, 1), 5);
    }

    #[test]
    fn subarray_sum_finds_target() {
        let arr = vec![1, -20, -3, 30, 5, 4];
        assert_eq!(subarray_sum(arr, 7), vec![1, 4]);
    }

    #[test]
    fn subarray_sum_no_solution() {
        let arr = vec![1, 2, 3, 4];
        assert_eq!(subarray_sum(arr, 8), vec![]);
    }

    #[test]
    fn subarray_sum_zero_length() {
        let arr = vec![];
        assert_eq!(subarray_sum(arr, 5), vec![]);
    }

    #[test]
    fn subarray_sum_total_same_elements() {
        let arr = vec![1, 1, 1];
        assert_eq!(subarray_sum_total(arr, 2), 2);
    }

    #[test]
    fn subarray_sum_total_with_negative() {
        let arr = vec![10, 5, -5, -20, 10];
        assert_eq!(subarray_sum_total(arr, -10), 3);
    }

    #[test]
    fn product_of_array_except_self_typical_case() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(product_of_array_except_self(&nums), vec![24, 12, 8, 6]);
    }

    #[test]
    fn product_of_array_except_self_with_zero() {
        let nums = vec![0, 1, 2, 3];
        assert_eq!(product_of_array_except_self(&nums), vec![6, 0, 0, 0]);
    }

    #[test]
    fn product_of_array_except_self_two_zeros() {
        let nums = vec![0, 1, 0, 3];
        assert_eq!(
            product_of_array_except_self(nums.as_ref()),
            vec![0, 0, 0, 0]
        );
    }

    #[test]
    fn product_of_array_except_self_single_element() {
        let nums = vec![5];
        assert_eq!(product_of_array_except_self(nums.as_ref()), vec![1]);
    }

    #[test]
    fn range_sum_query_immutable_typical_case() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(range_sum_query_immutable(nums, 1, 3), 9);
    }

    #[test]
    fn range_sum_query_immutable_single_element() {
        let nums = vec![5];
        assert_eq!(range_sum_query_immutable(nums, 0, 0), 5);
    }

    #[test]
    fn range_sum_query_immutable_entire_array() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(range_sum_query_immutable(nums, 0, 4), 15);
    }

    #[test]
    #[should_panic]
    fn range_sum_query_immutable_out_of_bounds() {
        let nums = vec![1, 2, 3];
        range_sum_query_immutable(nums, 0, 3);
    }
}
