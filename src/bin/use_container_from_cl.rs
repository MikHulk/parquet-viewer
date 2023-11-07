use std::error::Error;

use polars::prelude::*;

use parquet_viewer::Container;

fn main() -> Result<(), Box<dyn Error>> {
    let c = Container::new();
    println!("{}", c.to_str());
    let df = df!(
        "a" => &[1, 2, 3, 4, 5],
        "b" => &[6.0, 7.1, 8.2, 9.3, 0.4],
    )
    .unwrap();
    println!("{}", df);
    println!("{}", df[0]);
    println!("{:?}", df["a"].iter().collect::<Vec<_>>());
    println!("{:?}", df.clone().lazy().select([col("b")]).collect()?);
    for item in df.clone().lazy().select([col("b")]).collect()?.iter() {
        println!("{}", item);
    }
    Ok(())
}
