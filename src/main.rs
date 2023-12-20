use fred_rs::client::FredClient;
use fred_rs::series::observation::{Builder, Units, Frequency, Response};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new FredClient
    let mut client = FredClient::new()?;

    // Set the API key for the client
    client.with_key("fcb6d65823793e080edbddf6a62dfa19");

    // Create the argument builder
    let mut builder = Builder::new();

    // Set the arguments for the builder
    builder
        .observation_start("1930-01-01")
        .units(Units::PCH)
        .frequency(Frequency::A);

    // Make the request and pass in the builder to apply the arguments
    let resp: Response = client.series_observation("GNPCA", Some(builder))?;

    // Process the observations
    for data_point in &resp.observations {
        println!("Date: {}, Value: {}", data_point.date, data_point.value);
    }

    Ok(())
}
