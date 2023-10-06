mod b_rand;
pub mod linked_list;

use std::thread;

fn bubble_sort<T>(collection: &[T]) -> Vec<T>
where
    T: Ord + Clone,
{
    let mut sorted: Vec<T> = Vec::from(collection);

    let mut is_sorted = true;
    for p in 0..sorted.len() {
        for j in 0..sorted.len() - 1 - p {
            if sorted[j] > sorted[j + 1] {
                sorted.swap(j, j + 1);
                is_sorted = false;
            }
        }
        if is_sorted {
            break;
        }
    }

    sorted
}

fn bubble_sort_v2<T>(collection: &mut [T])
where
    T: Ord,
{
    let mut is_sorted = true;
    for p in 0..collection.len() {
        for j in 0..collection.len() - 1 - p {
            if collection[j] > collection[j + 1] {
                collection.swap(j, j + 1);
                is_sorted = false;
            }
        }
        if is_sorted {
            break;
        }
    }
}

fn merge_sort<T: Ord + Clone + Copy>(collection: Vec<T>) -> Vec<T> {
    if collection.len() <= 1 {
        return collection;
    }

    let mid = collection.len() / 2;

    let mut left = collection;
    let right = left.split_off(mid);

    return merge_arrays(merge_sort(left), merge_sort(right));
}

fn merge_arrays_v2<'a, T: Ord + Clone + Copy>(mut left: Vec<T>, mut right: Vec<T>) -> Vec<T> {
    let capacity = left.len() + right.len();
    let mut result: Vec<T> = Vec::with_capacity(capacity);

    let mut left_iter = left.into_iter();
    let mut right_iter = right.into_iter();

    let mut l_option = left_iter.next();
    let mut r_option = right_iter.next();

    while l_option.is_some() && r_option.is_some() {
        let l_v = l_option.unwrap();
        let r_v = r_option.unwrap();

        if l_v < r_v {
            result.push(l_v);
            l_option = left_iter.next();
        } else {
            result.push(r_v);
            r_option = right_iter.next();
        }
    }

    if let Some(l) = l_option {
        result.push(l);
        result.extend(left_iter);
    }

    if let Some(r) = r_option {
        result.push(r);
        result.extend(right_iter);
    }

    result
}

fn merge_arrays<T: Ord>(mut left: Vec<T>, mut right: Vec<T>) -> Vec<T> {
    let capacity = left.len() + right.len();
    let mut result: Vec<T> = Vec::with_capacity(capacity);

    let mut left_iter = left.into_iter();
    let mut right_iter = right.into_iter();
    let mut r_option = right_iter.next();

    let mut l_option = left_iter.next();

    loop {
        match l_option {
            Some(ref l_item) => match r_option {
                Some(ref r_item) => {
                    if l_item < r_item {
                        result.push(l_option.take().unwrap());
                        l_option = left_iter.next();
                        continue;
                    } else {
                        result.push(r_option.take().unwrap());
                        r_option = right_iter.next();
                    }
                }
                None => {
                    result.push(l_option.take().unwrap());
                    result.extend(left_iter);
                    break;
                }
            },
            None => {
                result.push(r_option.take().unwrap());
                result.extend(right_iter);
                break;
            }
        }
    }

    result
}

fn quick_sort_v2<T: Ord>(mut collection: Vec<T>) -> Vec<T> {
    if collection.len() <= 1 {
        return collection;
    }

    let capacity = (collection.len() / 2 + collection.len() % 2);

    let mut left = Vec::with_capacity(capacity);
    let mut right = Vec::with_capacity(capacity);

    let mut iterator = collection.into_iter();

    let pivot = iterator.next().unwrap();

    while let Some(element) = iterator.next() {
        if element < pivot {
            left.push(element);
        } else {
            right.push(element);
        }
    }

    let mut left = quick_sort_v2(left);
    let right = quick_sort_v2(right);

    left.push(pivot);
    left.extend(right);

    left
}

fn quick_sort<T: PartialOrd>(collection: &mut [T]) {
    if collection.len() <= 1 {
        return;
    }

    let pivot = get_pivot(collection);

    let (left, right) = collection.split_at_mut(pivot);

    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn get_pivot<T: PartialOrd>(collection: &mut [T]) -> usize {
    let mut pivot = b_rand::rand(collection.len());
    collection.swap(pivot, 0);
    pivot = 0;

    for i in 1..collection.len() {
        if collection[i] < collection[pivot] {
            collection.swap(pivot + 1, i);
            collection.swap(pivot, pivot + 1);
            pivot + 1;
        }
    }
    pivot
}

fn thread_quick_sort<T: Send + 'static + PartialOrd>(collection: &mut [T]) {
    if collection.len() <= 1 {
        return;
    }

    let pivot = get_pivot(collection);

    let (left, right) = collection.split_at_mut(pivot);

    thread::scope(|scope| {
        let h = scope.spawn(|| {
            thread_quick_sort(left);
        });

        thread_quick_sort(&mut right[1..]);
    });
}

#[cfg(test)]
mod tests {
    use crate::{
        bubble_sort, bubble_sort_v2, merge_sort, quick_sort, quick_sort_v2, thread_quick_sort,
    };

    #[test]
    fn test_bubble_sort() {
        let mut array = [100, 7, 6, 88, 5, 1, 2, 3, 4, 7];
        let sorted = [1, 2, 3, 4, 5, 6, 7, 7, 88, 100];

        assert_eq!(bubble_sort(&array), sorted);

        bubble_sort_v2(&mut array);
        assert_eq!(array, sorted);
    }

    #[test]
    fn merge_sort_sort() {
        let array = vec![100000, 100, 7, 6, 88, 5, 400000000, 1, 2, 3, 4, 7];
        let sorted = vec![1, 2, 3, 4, 5, 6, 7, 7, 88, 100, 100000, 400000000];

        assert_eq!(merge_sort(array), sorted);
    }

    #[test]
    fn quick_sort_test() {
        let array = vec![100000, 100, 7, 6, 88, 5, 400000000, 1, 2, 3, 4, 7];
        let sorted = vec![1, 2, 3, 4, 5, 6, 7, 7, 88, 100, 100000, 400000000];

        assert_eq!(quick_sort_v2(array), sorted);

        let mut array = vec![100000, 100, 7, 6, 88, 5, 400000000, 1, 2, 3, 4, 7];
        let sorted = vec![1, 2, 3, 4, 5, 6, 7, 7, 88, 100, 100000, 400000000];

        quick_sort(&mut array[..]);
        assert_eq!(array, sorted);
    }

    #[test]
    fn threader_quick_sort_test() {
        let mut array = vec![100000, 100, 7, 6, 88, 5, 400000000, 1, 2, 3, 4, 7];
        let sorted = vec![1, 2, 3, 4, 5, 6, 7, 7, 88, 100, 100000, 400000000];

        thread_quick_sort(&mut array[..]);
        assert_eq!(array, sorted);
    }
}
