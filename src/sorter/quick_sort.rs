use super::sorter_trait::Sorter;

pub struct QuickSort;

fn quicksort<T: PartialOrd + std::fmt::Debug>(v: &mut [T]) {
    match v.len() {
        0 | 1 => return (),
        2 => {
            if v[0] > v[1] {
                v.swap(0, 1);
            }
            return ();
        }
        _ => {}
    }

    let (pivot, rest) = v.split_first_mut().unwrap();

    let mut left = 0;
    let mut right = rest.len() - 1;

    while left <= right {
        if rest[left] >= *pivot {
            rest.swap(left, right);
            if right == 0 {
                break;
            }
            right -= 1;
        } else if rest[right] >= *pivot {
            left += 1;
            right -= 1;
        } else {
            left += 1;
        }
    }

    let right = right + 1;
    let mut last_swap = false;
    if right == 1 {
        if rest[right - 1] < *pivot {
            v.swap(0, right);
            last_swap = true;
        }
    } else {
        v.swap(0, right);
        last_swap = true;
    }

    if last_swap {
        let l = &v[0..right];
        let r = &v[right + 1..];
        if l.len() < r.len() {
            quicksort(&mut v[0..right]);
            quicksort(&mut v[right + 1..]);
        } else {
            quicksort(&mut v[right + 1..]);
            quicksort(&mut v[0..right]);
        }
    } else {
        quicksort(&mut v[right..]);
    }
    println!("order => {:?}", v);
}

impl<T: PartialOrd + std::fmt::Debug> Sorter<T> for QuickSort {
    fn sort(self, v: &mut [T]) {
        quicksort(v);
    }
}
