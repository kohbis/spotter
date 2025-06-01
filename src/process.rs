use prettytable::{Cell, Row, Table};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct InstanceInfo {
    pub interruption_rate: String,
    pub savings: String,
    pub linux_spot_price: String,
    pub windows_spot_price: String,
    pub memory_gb: String,
    pub cores: String,
}

pub fn display_spot_data(
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
