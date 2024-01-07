pub mod sorter;

#[cfg(test)]
mod tests {
    use super::sorter::{insertion_sort::InsertionSort, bubble_sort::BubbleSort, selection_sort::SelectionSort};
    use super::sorter::sorter_trait::Sorter;


    #[test]
    fn insert_sort_works() {
        let mut to_sort = [3,1,2,5,5,4];
        let mut expected = to_sort.clone();
        expected.sort();

        InsertionSort.sort(&mut to_sort[..]);

        assert_eq!(to_sort, expected);
    }

    #[test]
    fn bubble_sort_works() {
        let mut to_sort = [3,1,2,5,5,4];
        let mut expected = to_sort.clone();
        expected.sort();

        BubbleSort.sort(&mut to_sort[..]);

        assert_eq!(to_sort, expected);
    }

    #[test]
    fn selection_sort_works() {
        let mut to_sort = [3,1,2,5,5,4];
        let mut expected = to_sort.clone();
        expected.sort();

        SelectionSort.sort(&mut to_sort[..]);

        assert_eq!(to_sort, expected);
    }
}
