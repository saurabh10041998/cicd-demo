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
    fn run_tc1() {
        let arr = vec![5, 6, 3, 2, 4, 1, 2];
        assert_eq!(rev_sort(arr), vec![6, 5, 4, 3, 2, 2, 1]);
    }
}

fn main() {
    let arr = vec![5, 6, 3, 2, 4, 1, 2];
    assert_eq!(rev_sort(arr), vec![6, 5, 4, 3, 2, 2, 1]);
}
