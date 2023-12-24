use password_manager::api::fetch_and_process_data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fetch_and_process_data("GNPCA")?; // Real Gross National Product
    fetch_and_process_data("B230RC0A052NBEA")?; // Population
    fetch_and_process_data("FPCPITOTLZGUSA")?; // Inflation, consumer prices for the United States
    fetch_and_process_data("A065RC1A027NBEA")?; // Personal income

    Ok(())
}
