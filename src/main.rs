use clap::{App, Arg};
use password_manager::api::fetch_and_process_data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define command-line arguments using clap
    let matches = App::new("Economic Data Fetcher")
        .version("1.0")
        .author("Nick Stambaugh")
        .about("Fetches economic data from FRED API")
        .arg(
            Arg::with_name("series_id")
                .short("s")
                .long("series_id")
                .value_name("SERIES_ID")
                .help("Specifies the series ID to fetch data for")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    // Fetch data based on the provided series ID
    if let Some(series_id) = matches.value_of("series_id") {
        fetch_and_process_data(series_id)?;
    }

    Ok(())
}
