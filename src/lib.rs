use std::mem::swap;

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

#[cfg(test)]
mod tests {
    use crate::{bubble_sort, bubble_sort_v2, merge_sort};

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
}