use super::sorter_trait::Sorter;

pub struct SelectionSort;

impl<T: PartialOrd + std::fmt::Debug> Sorter<T> for SelectionSort {
    fn sort(self, v: &mut [T]) {
        for i in 0..v.len() - 1 {
            for j in i + 1..v.len() {
                if v[j] < v[i] {
                    v.swap(i, j)
                }
            }
        }
    }
}
