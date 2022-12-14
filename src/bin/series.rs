use starfish::Series;

fn main() {
    let data = vec![1.0, 5.0, 3.5, 2.9, 8.5];
    let index: Vec<i32> = (0..10).collect();
    let series: Series<f32, i32> = Series::new(data, index);

    // print index and elements in series
    for (i, d) in series.index.iter().zip(series.data.iter()) {
        println!("{}: {}", i, d);
    }
}
