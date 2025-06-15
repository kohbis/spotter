use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub const SPOT_ADVISOR_DATA_URL: &str =
    "https://spot-bid-advisor.s3.amazonaws.com/spot-advisor-data.json";
pub const SPOT_PRICE_DATA_URL: &str = "http://spot-price.s3.amazonaws.com/spot.js";

pub async fn fetch_spot_advisor_data(client: &Client) -> Result<Value, Box<dyn Error>> {
    log::info!("Fetching spot advisor data...");
    let url = SPOT_ADVISOR_DATA_URL;
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

pub async fn fetch_spot_price_data(client: &Client) -> Result<Value, Box<dyn Error>> {
    log::info!("Fetching spot price data...");
    let url = SPOT_PRICE_DATA_URL;
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // This test mocks the process of extracting JSON from the callback function
    #[test]
    fn test_extract_json_from_callback() {
        let callback_text = "callback({\"key\": \"value\"})";

        // Extract JSON from callback function
        let callback_prefix = "callback(";
        let start_idx = callback_text
            .find(callback_prefix)
            .map(|idx| idx + callback_prefix.len())
            .unwrap_or_else(|| callback_text.find('{').unwrap_or(0));

        let end_idx = callback_text.rfind('}').unwrap_or(callback_text.len());
        let json_str = &callback_text[start_idx..=end_idx];

        // Remove trailing parenthesis if present
        let json_str = if json_str.ends_with(')') {
            &json_str[..json_str.len() - 1]
        } else {
            json_str
        };

        let data = serde_json::from_str::<Value>(json_str).unwrap();

        assert_eq!(data, json!({"key": "value"}));
    }
}
