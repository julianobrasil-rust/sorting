pub trait Sorter<T: PartialOrd + std::fmt::Debug> {
    fn sort(self, v: &mut [T]);
}
