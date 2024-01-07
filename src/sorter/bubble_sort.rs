use super::sorter_trait::Sorter;

pub struct BubbleSort;

impl<T: PartialOrd + std::fmt::Debug> Sorter<T> for BubbleSort {
    fn sort(self, v: &mut [T]) {
        let mut swaped = true;
        while swaped {
            swaped = false;
            for i in 1..v.len() {
                if v[i] < v[i - 1] {
                    v.swap(i, i -1 );
                    swaped = true;
                }
            }
        }
    }
}