pub mod sorter;

#[cfg(test)]
mod tests {
    use crate::sorter::quick_sort::QuickSort;

    use super::sorter::sorter_trait::Sorter;
    use super::sorter::{
        bubble_sort::BubbleSort, insertion_sort::InsertionSort, selection_sort::SelectionSort,
    };

    #[test]
    fn insert_sort_works() {
        let mut to_sort = [3, 1, 2, 5, 5, 4];
        let mut expected = to_sort.clone();
        expected.sort();

        InsertionSort.sort(&mut to_sort[..]);

        assert_eq!(to_sort, expected);
    }

    #[test]
    fn bubble_sort_works() {
        let mut to_sort = [3, 1, 2, 5, 5, 4];
        let mut expected = to_sort.clone();
        expected.sort();

        BubbleSort.sort(&mut to_sort[..]);

        assert_eq!(to_sort, expected);
    }

    #[test]
    fn selection_sort_works() {
        let mut to_sort = [3, 1, 2, 5, 5, 4];
        let mut expected = to_sort.clone();
        expected.sort();

        SelectionSort.sort(&mut to_sort[..]);

        assert_eq!(to_sort, expected);
    }

    #[test]
    fn quick_sort_works() {
        let mut to_sort = [7, 3, 2, 1];
        let mut expected = to_sort.clone();
        expected.sort();
        println!("INITIAL => {:?}", to_sort);
        QuickSort.sort(&mut to_sort[..]);

        assert_eq!(to_sort, expected);
    }
}
