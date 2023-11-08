use std::error::Error;

use parquet_viewer::Container;

fn main() -> Result<(), Box<dyn Error>> {
    let c = Container::new();
    println!("{}", c.to_str());
    let df = c.get_data();
    println!("{}", df);
    println!("{:?}", df.get_column_names());
    for idx in 0..df.height() {
        let row = df.get_row(idx)?;
        for val in row.0.iter() {
            print!("| ");
            print!("{}", val);
        }
        println!(" |");
    }
    Ok(())
}
