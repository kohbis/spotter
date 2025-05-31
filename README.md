# Spotter

[![Crates.io](https://img.shields.io/crates/v/spotter.svg)](https://crates.io/crates/spotter)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A command-line tool for AWS EC2 Spot Instance Advisor. Spotter helps you find the most cost-effective and reliable spot instances across AWS regions.

## Features

- 🔍 View spot instance interruption rates and savings percentages
- 🌐 Support for all AWS regions
- 🔎 Filter by instance type, family, or size
- 📊 Tabular output for easy comparison

## Installation

### From Crates.io

```bash
cargo install spotter
```

### From Source

```bash
git clone https://github.com/kohbis/spotter.git
cd spotter
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Show spot instance information for the default region (us-east-1)
spotter

# Show spot instance information for a specific region
spotter --region ap-northeast-1
```

### Filtering by Instance Type

```bash
# Filter by instance family
spotter --instance-type m5

# Filter by instance size
spotter --instance-type large

# Filter by specific instance type
spotter --instance-type m5.large
```

### Show Spot Prices

> [!WARNING]
> For the latest and exact spot prices, check AWS management console.

```bash
# Include spot prices for Linux and Windows
spotter --spot-price
```

### Help

```bash
spotter --help
```

## Example Output

```
+---------------+--------------+-------------------+-----------+-------+------------------+--------------------+---------+
| Instance Type | Region       | Interruption Rate | Memory GB | Cores | Linux Spot Price | Windows Spot Price | Savings |
+---------------+--------------+-------------------+-----------+-------+------------------+--------------------+---------+
| c5.large      | us-east-1    | < 5%              | 4.0       | 2     | 0.0431           | 0.1431             | 72%     |
| m5.large      | us-east-1    | 5-10%             | 8.0       | 2     | 0.0452           | 0.1452             | 68%     |
| r5.large      | us-east-1    | < 5%              | 16.0      | 2     | 0.0595           | 0.1595             | 70%     |
+---------------+--------------+-------------------+-----------+-------+------------------+--------------------+---------+
```

## How It Works

Spotter fetches data from two AWS sources:
1. [**Spot Advisor Data**](https://spot-bid-advisor.s3.amazonaws.com/spot-advisor-data.json): Provides information about interruption rates and savings percentages
2. [**Spot Price Data**](https://spot-price.s3.amazonaws.com/spot.js): Provides current spot prices for different instance types

The tool combines this information to give you a comprehensive view of spot instances, helping you make informed decisions about which instances to use for your workloads.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

[kohbis](https://github.com/kohbis)
