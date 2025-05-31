use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use prettytable::{Cell, Row, Table};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// AWS region (default: us-east-1)
    #[arg(short, long, default_value = "us-east-1")]
    region: String,

    /// EC2 instance type to filter by (family like 'm5', size like 'large', or full type like 'm5.large')
    #[arg(short, long)]
    instance_type: Option<String>,

    /// Show spot prices for Linux and Windows (for latest pricing information, check AWS Management Console)
    #[arg(long)]
    spot_price: bool,

    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    // Fetch data from both sources
    let client = Client::new();
    let advisor_data = fetch_spot_advisor_data(&client).await?;
    let price_data = fetch_spot_price_data(&client).await?;

    // Process and display the data
    display_spot_data(
        &cli.region,
        cli.instance_type.as_deref(),
        &advisor_data,
        &price_data,
        cli.spot_price,
    )?;

    Ok(())
}

async fn fetch_spot_advisor_data(client: &Client) -> Result<Value, Box<dyn Error>> {
    log::info!("Fetching spot advisor data...");
    let url = "https://spot-bid-advisor.s3.amazonaws.com/spot-advisor-data.json";
    let response = client.get(url).send().await?;
    let data = response.json::<Value>().await?;

    // Print a sample of the data structure
    if let Some(spot_advisor) = data.get("spot_advisor") {
        if let Some(regions) = spot_advisor.as_object() {
            if let Some((region_name, region_data)) = regions.iter().next() {
                log::debug!("Sample region from advisor data: {}", region_name);
                if let Some(instances) = region_data.as_object() {
                    log::debug!(
                        "Number of instance entries in region {}: {}",
                        region_name,
                        instances.len()
                    );

                    // Print a few instance names to understand the structure
                    for (i, (instance_name, instance_info)) in instances.iter().enumerate() {
                        if i < 5 {
                            log::debug!("  Instance {}: {}", i + 1, instance_name);
                            if let Some(info) = instance_info.as_object() {
                                if let Some(rate) = info.get("r") {
                                    log::debug!("    Rate value: {}", rate);
                                }
                                if let Some(savings) = info.get("s") {
                                    log::debug!("    Savings value: {}", savings);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Check for instance_types data structure
    if let Some(instance_types) = data.get("instance_types") {
        if let Some(instance_types_obj) = instance_types.as_object() {
            log::debug!(
                "Found instance_types with {} entries",
                instance_types_obj.len()
            );
            // Print a few examples
            for (i, (instance_name, instance_info)) in instance_types_obj.iter().enumerate() {
                if i < 3 {
                    log::debug!("  Instance type {}: {}", i + 1, instance_name);
                    if let Some(info) = instance_info.as_object() {
                        if let Some(cores) = info.get("cores") {
                            log::debug!("    Cores: {}", cores);
                        }
                        if let Some(ram_gb) = info.get("ram_gb") {
                            log::debug!("    RAM GB: {}", ram_gb);
                        }
                        if let Some(emr) = info.get("emr") {
                            log::debug!("    EMR: {}", emr);
                        }
                    }
                }
            }
        }
    }

    Ok(data)
}

async fn fetch_spot_price_data(client: &Client) -> Result<Value, Box<dyn Error>> {
    log::info!("Fetching spot price data...");
    let url = "http://spot-price.s3.amazonaws.com/spot.js";
    let response = client.get(url).send().await?;
    let text = response.text().await?;

    // Extract JSON from callback function
    let callback_prefix = "callback(";
    let start_idx = text
        .find(callback_prefix)
        .map(|idx| idx + callback_prefix.len())
        .unwrap_or_else(|| text.find('{').unwrap_or(0));

    let end_idx = text.rfind('}').unwrap_or(text.len());
    let json_str = &text[start_idx..=end_idx];

    // Remove trailing parenthesis if present
    let json_str = if json_str.ends_with(')') {
        &json_str[..json_str.len() - 1]
    } else {
        json_str
    };

    let data = serde_json::from_str::<Value>(json_str)?;

    // Print a sample of the data structure
    if let Some(config) = data.get("config") {
        if let Some(regions) = config.get("regions") {
            if let Some(regions_array) = regions.as_array() {
                if !regions_array.is_empty() {
                    if let Some(region_name) = regions_array[0].get("region") {
                        log::debug!("Sample region from price data: {}", region_name);
                    }

                    // Print more detailed information about the price data structure
                    if let Some(instance_types) = regions_array[0].get("instanceTypes") {
                        if let Some(instance_types_array) = instance_types.as_array() {
                            if !instance_types_array.is_empty() {
                                if let Some(instance_type) = instance_types_array[0].get("type") {
                                    log::debug!(
                                        "Sample instance type from price data: {}",
                                        instance_type
                                    );

                                    // Print information about sizes
                                    if let Some(sizes) = instance_types_array[0].get("sizes") {
                                        if let Some(sizes_array) = sizes.as_array() {
                                            if !sizes_array.is_empty() {
                                                log::debug!(
                                                    "  Number of sizes: {}",
                                                    sizes_array.len()
                                                );
                                                if let Some(size_name) = sizes_array[0].get("size")
                                                {
                                                    log::debug!("  Sample size: {}", size_name);

                                                    // Print information about value columns
                                                    if let Some(value_columns) =
                                                        sizes_array[0].get("valueColumns")
                                                    {
                                                        if let Some(value_columns_array) =
                                                            value_columns.as_array()
                                                        {
                                                            log::debug!(
                                                                "    Number of value columns: {}",
                                                                value_columns_array.len()
                                                            );
                                                            for (i, value_column) in
                                                                value_columns_array
                                                                    .iter()
                                                                    .enumerate()
                                                            {
                                                                if i < 3 {
                                                                    if let Some(name) =
                                                                        value_column.get("name")
                                                                    {
                                                                        log::debug!(
                                                                            "    Value column {}: {}",
                                                                            i + 1,
                                                                            name
                                                                        );

                                                                        // Check if there's spot price information
                                                                        if let Some(_spot) =
                                                                            value_column.get("spot")
                                                                        {
                                                                            log::debug!(
                                                                                "      Has spot price information"
                                                                            );
                                                                        } else {
                                                                            log::debug!(
                                                                                "      No spot price information"
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(data)
}

fn display_spot_data(
    region: &str,
    instance_type: Option<&str>,
    advisor_data: &Value,
    price_data: &Value,
    show_spot_price: bool,
) -> Result<(), Box<dyn Error>> {
    // Create a table to display the data
    let mut table = Table::new();

    // Add table headers
    let mut headers = vec![
        Cell::new("Instance Type"),
        Cell::new("Region"),
        Cell::new("Interruption Rate"),
        Cell::new("Memory (GB)"),
        Cell::new("Cores"),
    ];

    if show_spot_price {
        headers.push(Cell::new("Linux Spot Price"));
        headers.push(Cell::new("Windows Spot Price"));
    }

    headers.push(Cell::new("Savings"));
    table.add_row(Row::new(headers));

    log::info!("Processing spot instance data...");

    // Extract instance specifications from the instance_types data
    let mut instance_specs: HashMap<String, (String, String)> = HashMap::new();
    if let Some(instance_types) = advisor_data.get("instance_types") {
        if let Some(instance_types_obj) = instance_types.as_object() {
            log::debug!(
                "Processing {} instance type specifications",
                instance_types_obj.len()
            );
            for (instance_name, instance_info) in instance_types_obj {
                if let Some(info) = instance_info.as_object() {
                    let ram_gb = info
                        .get("ram_gb")
                        .and_then(Value::as_f64)
                        .map(|r| r.to_string())
                        .unwrap_or_else(|| "N/A".to_string());
                    let cores = info
                        .get("cores")
                        .and_then(Value::as_u64)
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| "N/A".to_string());

                    instance_specs.insert(instance_name.clone(), (ram_gb, cores));
                }
            }
        }
    }

    // Process advisor data
    let advisor_regions = advisor_data["spot_advisor"].as_object().unwrap();
    log::debug!(
        "Number of regions in advisor data: {}",
        advisor_regions.len()
    );

    // Check if the specified region exists in advisor data
    if !advisor_regions.contains_key(region) {
        log::debug!("Warning: Region '{}' not found in advisor data", region);
        log::debug!(
            "Available regions in advisor data: {:?}",
            advisor_regions.keys().collect::<Vec<_>>()
        );
    }

    // Process price data
    let price_regions = price_data["config"]["regions"].as_array().unwrap();
    log::debug!("Number of regions in price data: {}", price_regions.len());

    // Check if the specified region exists in price data
    let region_exists_in_price_data = price_regions
        .iter()
        .any(|r| r.get("region").and_then(Value::as_str) == Some(region));

    if !region_exists_in_price_data {
        log::debug!("Warning: Region '{}' not found in price data", region);
        log::debug!(
            "Available regions in price data: {:?}",
            price_regions
                .iter()
                .filter_map(|r| r.get("region").and_then(Value::as_str))
                .collect::<Vec<_>>()
        );
    }

    // Create a mapping of instance types to their data
    let mut instance_data: HashMap<String, HashMap<String, InstanceInfo>> = HashMap::new();

    // Debug counter for instance types
    let mut instance_count = 0;

    // Process advisor data - we need to handle the structure differently
    // The advisor data has Linux/Windows at the top level, and then instance types under them
    for (region_name, region_data) in advisor_regions {
        let region_map = region_data.as_object().unwrap();

        if region_name == region {
            log::debug!(
                "Found region '{}' in advisor data with {} entries",
                region_name,
                region_map.len()
            );
        }

        // Check if Linux exists in the region data
        if let Some(linux_data) = region_map.get("Linux") {
            if let Some(linux_instances) = linux_data.as_object() {
                log::debug!(
                    "Found Linux instances for region {}: {}",
                    region_name,
                    linux_instances.len()
                );

                // Process Linux instances
                for (instance_name, instance_info) in linux_instances {
                    instance_count += 1;

                    if let Some(info) = instance_info.as_object() {
                        let rate_info = info.get("r").and_then(Value::as_u64).unwrap_or(0);
                        let savings = info.get("s").and_then(Value::as_u64).unwrap_or(0);

                        // Map rate_info to a descriptive string
                        let interruption_rate = match rate_info {
                            0 => "< 5%",
                            1 => "5-10%",
                            2 => "10-15%",
                            3 => "15-20%",
                            _ => "> 20%",
                        };

                        // Create or get the region map for this instance type
                        let region_map = instance_data
                            .entry(instance_name.clone())
                            .or_insert_with(HashMap::new);

                        // Get memory and cores information from instance_specs
                        let (memory_gb, cores) = instance_specs
                            .get(instance_name)
                            .map(|(ram, cores)| (ram.clone(), cores.clone()))
                            .unwrap_or_else(|| ("N/A".to_string(), "N/A".to_string()));

                        // Insert or update the instance info for this region
                        region_map.insert(
                            region_name.clone(),
                            InstanceInfo {
                                interruption_rate: interruption_rate.to_string(),
                                savings: format!("{}%", savings),
                                linux_spot_price: "N/A".to_string(),
                                windows_spot_price: "N/A".to_string(),
                                memory_gb: memory_gb.clone(),
                                cores: cores.clone(),
                            },
                        );
                    }
                }
            }
        }
    }

    log::info!(
        "Total instance types found in advisor data: {}",
        instance_count
    );

    // Debug counter for price data
    let mut price_instance_count = 0;

    // Process price data
    for region_data in price_regions {
        let region_name = region_data["region"].as_str().unwrap();
        let instance_types = region_data["instanceTypes"].as_array().unwrap();

        if region_name == region {
            log::debug!(
                "Found region '{}' in price data with {} instance type categories",
                region_name,
                instance_types.len()
            );
        }

        for instance_type_data in instance_types {
            let instance_type_name = instance_type_data["type"].as_str().unwrap();
            let sizes = instance_type_data["sizes"].as_array().unwrap();

            if region_name == region {
                log::debug!(
                    "  Instance type category '{}' has {} sizes",
                    instance_type_name,
                    sizes.len()
                );
            }

            for size in sizes {
                let size_name = size["size"].as_str().unwrap();
                let full_name = format!("{}.{}", instance_type_name, size_name);

                // Extract simple instance name (e.g., "m5.large" from "generalCurrentGen.m5.large")
                let simple_name = if let Some(last_dot_index) = full_name.rfind('.') {
                    if let Some(second_last_dot_index) = full_name[..last_dot_index].rfind('.') {
                        full_name[second_last_dot_index + 1..].to_string()
                    } else {
                        full_name.clone()
                    }
                } else {
                    full_name.clone()
                };

                price_instance_count += 1;

                if region_name == region && price_instance_count <= 5 {
                    log::debug!(
                        "    Instance type: {} -> simple: {}",
                        full_name,
                        simple_name
                    );
                }

                let values = size["valueColumns"].as_array().unwrap();
                let mut linux_spot_price = "N/A".to_string();
                let mut windows_spot_price = "N/A".to_string();

                for value in values {
                    let name = value["name"].as_str().unwrap();

                    // Get Linux spot price
                    if name == "linux" {
                        // Try to get spot price if available
                        if let Some(spot_values) = value.get("prices") {
                            if let Some(spot_prices) = spot_values.get("USD") {
                                linux_spot_price =
                                    spot_prices.as_str().unwrap_or("N/A").to_string();
                            }
                        }
                    }

                    // Get Windows spot price
                    if name == "mswin" {
                        // Try to get spot price if available
                        if let Some(spot_values) = value.get("prices") {
                            if let Some(spot_prices) = spot_values.get("USD") {
                                windows_spot_price =
                                    spot_prices.as_str().unwrap_or("N/A").to_string();
                            }
                        }
                    }
                }

                // Update the instance info with price data using simple name
                if let Some(region_map) = instance_data.get_mut(&simple_name) {
                    if let Some(info) = region_map.get_mut(region_name) {
                        info.linux_spot_price = linux_spot_price.clone();
                        info.windows_spot_price = windows_spot_price.clone();

                        if region_name == region && price_instance_count <= 5 {
                            log::debug!(
                                "      Updated existing entry: {} with Linux price: {}",
                                simple_name,
                                linux_spot_price
                            );
                        }
                    }
                } else {
                    // Instance not found in advisor data, create a new entry
                    let mut region_map = HashMap::new();
                    region_map.insert(
                        region_name.to_string(),
                        InstanceInfo {
                            interruption_rate: "N/A".to_string(),
                            savings: "N/A".to_string(),
                            linux_spot_price,
                            windows_spot_price,
                            memory_gb: "N/A".to_string(),
                            cores: "N/A".to_string(),
                        },
                    );
                    instance_data.insert(simple_name, region_map);
                }
            }
        }
    }

    log::info!(
        "Total instance types found in price data: {}",
        price_instance_count
    );

    // Filter data based on region and instance type
    let mut filtered_data: Vec<(String, InstanceInfo)> = Vec::new();

    for (instance_name, region_map) in &instance_data {
        if let Some(filter_instance) = instance_type {
            // Check if the filter matches family or size
            // Instance name format: "family.size" (e.g., "m5.large")
            let parts: Vec<&str> = instance_name.split('.').collect();
            let family = parts.get(0).unwrap_or(&"");
            let size = parts.get(1).unwrap_or(&"");

            // Check if filter matches the family, size, or the whole instance name
            let matches = family == &filter_instance
                || size == &filter_instance
                || instance_name.contains(filter_instance);

            if !matches {
                continue;
            }
        }

        if let Some(info) = region_map.get(region) {
            filtered_data.push((instance_name.clone(), info.clone()));
        }
    }

    // Sort by instance name
    filtered_data.sort_by(|a, b| a.0.cmp(&b.0));

    // Add rows to the table
    for (instance_name, info) in filtered_data {
        let mut row_cells = vec![
            Cell::new(&instance_name),
            Cell::new(region),
            Cell::new(&info.interruption_rate),
            Cell::new(&info.memory_gb),
            Cell::new(&info.cores),
        ];

        if show_spot_price {
            row_cells.push(Cell::new(&info.linux_spot_price));
            row_cells.push(Cell::new(&info.windows_spot_price));
        }

        row_cells.push(Cell::new(&info.savings));

        table.add_row(Row::new(row_cells));
    }

    // Print the number of instances found
    log::info!(
        "Found {} spot instances for region: {}, filtering by instance type: {}",
        table.len() - 1,
        region,
        instance_type.unwrap_or("all")
    );

    // Print the table
    table.printstd();

    Ok(())
}

// Structure to hold instance information
#[derive(Clone, Debug)]
struct InstanceInfo {
    interruption_rate: String,
    savings: String,
    linux_spot_price: String,
    windows_spot_price: String,
    memory_gb: String,
    cores: String,
}
