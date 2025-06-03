use std::cmp::Reverse;
use std::collections::BinaryHeap;



/// Finds the three smallest elements in a vector using a max-heap.
///
/// This function accepts a `Vec<i32>` and returns a `Vec<i32>` containing the
/// three smallest elements from the input vector in ascending order. If the
/// input vector has fewer than three elements, it will return as many elements
/// as available, sorted in ascending order.
///
/// # Arguments
///
/// * `arr` - A vector of integers from which the smallest three elements will be extracted.
///
/// # Returns
///
/// * A vector of integers containing the three smallest elements from the input vector.
///   The result is sorted in ascending order.
///
/// # Panics
///
/// * The function will panic if the input vector contains fewer than three elements
///   and attempts to access `heap.pop().unwrap()`.
///
/// # Examples
///
/// ```
/// use std::collections::BinaryHeap;
/// use std::collections::BinaryHeap::Reverse;
///
/// let nums = vec![10, 1, 5, 2, 8, 3];
/// let smallest = heap_top_3_smallest(nums);
/// assert_eq!(smallest, vec![1, 2, 3]);
/// ```
#[allow(dead_code)]
fn heap_top_3_smallest(arr: Vec<i32>) -> Vec<i32> {
    
    // Reverse needs to be used, BinaryHeap is max-heap
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    for i in arr {
        heap.push(Reverse(i))
    }
    
    let mut result = Vec::with_capacity(3);
    for _ in 0..3 {
        result.push(heap.pop().unwrap().0);
    }
    
    result
    
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Point2D {
    pub x: i32,
    pub y: i32,
}
impl Point2D  {
    // there is no need to use sqrt, because it is still a monotonic function, no mather sqrt used or not. 
    // removing sqrt makes code simpler; f64 does not implement Ord required by BinaryHeap
    pub fn euclides_distance_to_center(&self) -> i32 {
        self.x.pow(2) + self.y.pow(2)
    }
}
/// Finds the k closest points to the origin (0, 0) in a given list of points.
///
/// This function calculates the Euclidean distance of each point from the origin
/// and uses a max-heap to efficiently determine the k closest points. Each point
/// is represented as a 2D coordinate (x, y).
///
/// # Arguments
/// * `points` - A `Vec<Vec<i32>>` where each inner vector contains two integers
///   representing the x and y coordinates of a point in a 2D plane.
/// * `k` - An `i32` representing the number of closest points to find.
///
/// # Returns
/// * A `Vec<Vec<i32>>` where each inner vector contains the x and y coordinates
///   of one of the k closest points to the origin.
///
/// # Example
/// ```rust
/// let points = vec![vec![1, 3], vec![-2, 2], vec![5, 8], vec![0, 1]];
/// let k = 2;
/// let result = k_closest_points(points, k);
/// assert_eq!(result, vec![vec![-2, 2], vec![0, 1]]);
/// ```
///
/// # Assumptions
/// * The input `points` vector contains valid 2D points with exactly two elements each.
/// * The value of `k` is less than or equal to the number of points in the input `points`.
///
/// # Panics
/// This function will panic if `heap.pop()` is called on an empty heap due to invalid `k`
/// or insufficient points.
///
/// # Implementation Notes
/// * The function uses a `BinaryHeap` to store points with their distances from the origin.
///   The `Reverse` wrapper is used to achieve a min-heap behavior.
/// * The points are sorted by their Euclidean distance to (0, 0).
/// * The `Point2D` struct and `euclides_distance_to_center` method are assumed to be
///   implemented elsewhere. `euclides_distance_to_center` computes the squared Euclidean
///   distance (avoiding the overhead of floating-point square root calculations).
///
/// # Caveats
/// * The function unnecessarily calls `heap.pop()` twice to extract x and y coordinates
///   from the same point, which can result in incorrect behavior and panics. This bug
///   needs to be fixed by saving and reusing the popped point instead of calling `heap.pop()` twice.
#[allow(dead_code)]
fn k_closest_points(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {

    let mut heap: BinaryHeap<Reverse<(i32, Point2D)>> = BinaryHeap::new();
    for i in points {
        let p = Point2D { x: i[0], y: i[1] };
        heap.push(Reverse((p.euclides_distance_to_center(), p)));
    }

    let mut result = Vec::with_capacity(k as usize);
    for _ in 0..k {
        let point = heap.pop().unwrap().0.1;
        result.push(vec![point.x, point.y]);
    }
    result

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_top_3_smallest_normal_input() {
        let nums = vec![10, 1, 5, 2, 8, 3];
        assert_eq!(heap_top_3_smallest(nums), vec![1, 2, 3]);
    }

    #[test]
    fn test_heap_top_3_smallest_duplicates() {
        let nums = vec![5, 2, 2, 1, 3, 1];
        assert_eq!(heap_top_3_smallest(nums), vec![1, 1, 2]);
    }

    #[test]
    #[should_panic]
    fn test_heap_top_3_smallest_empty() {
        let nums: Vec<i32> = vec![];
        heap_top_3_smallest(nums);
    }

    #[test]
    fn test_heap_top_3_smallest_exact_three() {
        let nums = vec![3, 2, 1];
        assert_eq!(heap_top_3_smallest(nums), vec![1, 2, 3]);
    }

    #[test]
    fn test_k_closest_points_normal() {
        let points = vec![vec![1, 3], vec![-2, 2], vec![5, 8], vec![0, 1]];
        let k = 2;
        let mut result = k_closest_points(points, k);
        result.sort(); // Sort for consistent comparison
        assert_eq!(result, vec![vec![-2, 2], vec![0, 1]]);
    }

}