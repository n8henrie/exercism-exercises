#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(bigger: &[T], smaller: &[T]) -> bool {
    if smaller.len() > bigger.len() {
        return false;
    }

    for idx in 0..=(bigger.len() - smaller.len()) {
        if &bigger[idx..idx + smaller.len()] == smaller {
            return true;
        }
    }
    false
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.iter().eq(second_list) {
        return Comparison::Equal;
    }

    if is_sublist(first_list, second_list) {
        Comparison::Superlist
    } else if is_sublist(second_list, first_list) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}
