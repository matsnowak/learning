#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.is_empty() && second_list.is_empty() {
        return Comparison::Equal;
    }

    if first_list.is_empty() {
        return Comparison::Sublist;
    }

    if second_list.is_empty() {
        return Comparison::Superlist;
    }

    if first_list.len() == second_list.len() {
        for (left, right) in first_list.iter().zip(second_list) {
            if left != right {
                return Comparison::Unequal;
            }
        }

        return Comparison::Equal;
    }

    if first_list.len() < second_list.len() {
        if is_sublist(first_list, second_list) {
            return Comparison::Sublist;
        }
    } else if is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

fn is_sublist(first_list: &[i32], second_list: &[i32]) -> bool {
    let mut second_idx = 0;

    while second_idx < second_list.len() {
        let mut l = 0;
        let mut r = second_idx;
        (l, r, first_list.len(), second_list.len());
        while l < first_list.len() && r < second_list.len() && first_list[l] == second_list[r] {
            l += 1;
            r += 1;
        }
        if l == first_list.len() {
            return true;
        }
        second_idx += 1;
    }
    false
}
