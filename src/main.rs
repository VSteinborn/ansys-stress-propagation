use polars::prelude::*;
use std::env;

fn read_file(file_name: &str) -> DataFrame {
    CsvReadOptions::default()
        .with_has_header(true)
        .with_parse_options(
            CsvParseOptions::default()
                .with_separator(b'\t')
                .with_decimal_comma(true) // German number system
            )
        .try_into_reader_with_file_path(Some(file_name.into()))
        .expect("Cannot read file.")
        .finish()
        .expect("Cannot write data to memory.")
}

fn calculate_cutoff_table(df: DataFrame, query: f32) -> DataFrame {
    df.lazy()
        .with_columns([
            col("Schubspannung (MPa)").gt(query).alias(format!("Greater Than {query} (MPa)"))
        ])
        .group_by([(col(format!("Greater Than {query} (MPa)")))])
        .agg([
            col("X-Position (mm)").max().alias("Max X (mm)"),
            col("X-Position (mm)").min().alias("Min X (mm)"),
            col("Y-Position (mm)").max().alias("Max Y (mm)"),
            col("Y-Position (mm)").min().alias("Min Y (mm)"),
            col("Z-Position (mm)").max().alias("Max Z (mm)"),
            col("Z-Position (mm)").min().alias("Min Z (mm)"),
            ])
        .collect()
        .expect("Could not extract dataframe")
}

fn main(){
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let query: f32 = query.trim().parse().expect("Please type a number!");
    let file_path = &args[2];
    
    let df: DataFrame = read_file(file_path);
    let lf = calculate_cutoff_table(df, query);

    println!("{}", lf);
}
