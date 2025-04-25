#![allow(dead_code)]
fn sort_list<T: Ord>(list: &mut [T]) {
    list.sort();
}

#[cfg(test)]
mod test {

    use super::sort_list;

    #[test]
    fn basic_rust_sort() {
        let mut input = vec![5, -1, 4, -2, 3, 0, 2, 1];
        sort_list::<i32>(&mut input);
        assert_eq!(input, vec![-2, -1, 0, 1, 2, 3, 4, 5]);
    }
}
