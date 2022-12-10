#[cfg(test)]
mod series_tests {
    use crate::Series;
    #[test]
    fn test_get() {
        // create a Series with i32 data
        let data = vec![5, 10, 15, 20, 25, 30];
        let index: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
        let series = Series::new(data, index);

        // test the get() method with a valid index label
        let mut element = series.get(&'b');
        assert_eq!(element, Some(&10));
        element = series.get(&'c');
        assert_eq!(element, Some(&15));

        // test the get() method with an invalid label
        let element = series.get(&'x');
        assert_eq!(element, None);
    }
}
