use std::collections::BinaryHeap;

fn rev_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut pq = arr.drain(..).collect::<BinaryHeap<i32>>();
    let mut ans = vec![];
    while let Some(x) = pq.pop() {
        ans.push(x);
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::rev_sort;
    #[test]
    fn test_rev_sort() {
        let arr = vec![2, 3, 6, 1, 2, 5];
        assert_eq!(rev_sort(arr), vec![6, 5, 3, 2, 2, 1]);
    }
}

fn main() {
    let arr = vec![2, 3, 6, 1, 2, 5];
    assert_eq!(rev_sort(arr), vec![6, 5, 3, 2, 2, 1]);
}
