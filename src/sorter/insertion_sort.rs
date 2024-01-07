use super::sorter_trait::Sorter;

pub struct InsertionSort;

impl<T: PartialOrd + std::fmt::Debug> Sorter<T> for InsertionSort {
    fn sort(self, v: &mut [T]) {
        println!("{:?}", v);
        for i in 1..v.len() {
            let mut j = i;
            while j > 0 && v[j] < v[j - 1] {
                v.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}
