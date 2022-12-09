use std::fmt::Display;

mod tests;

pub struct Series<I, T>
where
    I: Display + PartialEq,
    T: Display + PartialEq,
{
    // the index labels for the elements in the series
    pub index: Vec<I>,
    // the data for the series
    pub data: Vec<T>,
}

impl<I, T> Series<I, T>
where
    I: Display + PartialEq,
    T: Display + PartialEq,
{
    pub fn new(index: Vec<I>, data: Vec<T>) -> Series<I, T> {
        Series { index, data }
    }

    /// Retrieves the element in the `Series` with the given `index` label.
    ///
    /// # Arguments
    ///
    /// * `index` - A reference to the `I` type index label of the element to be retrieved.
    ///
    /// # Returns
    ///
    /// If the given `index` label exists in the `Series`, returns an `Option` value containing a
    /// reference to the corresponding element in the `Series`. If the `index label does not exist
    /// in the `Series`, then `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use starfish::Series;
    ///
    /// let data = vec![5, 10, 15, 20, 25, 30];
    /// let index: Vec<usize> = (0..data.len()).collect();
    /// let series: Series<usize, i32> = Series::new(index, data);
    ///
    /// let element = series.get(&2);
    /// assert_eq!(element, Some(&15));
    /// ```
    pub fn get(&self, index: &I) -> Option<&T> {
        // find the index of the given index label
        let i = self.index.iter().position(|x| x == index);

        // if the index label was found, return the corresponding element
        if let Some(i) = i {
            Some(&self.data[i])
        } else {
            // otherwise, return None
            None
        }
    }
}
