#[cfg(test)]
mod series_tests {
    use crate::Series;
    #[test]
    fn test_get() {
        // create a Series with i32 data
        let data = vec![5, 10, 15, 20, 25, 30];
        let index: Vec<usize> = (0..data.len()).collect();
        let series: Series<usize, i32> = Series::new(index, data);

        // test the get() method with a valid index label
        let mut element = series.get(&1);
        assert_eq!(element, Some(&10));
        element = series.get(&3);
        assert_eq!(element, Some(&20));

        // test the get() method with an invalid label
        let element = series.get(&17);
        assert_eq!(element, None);
    }
}
