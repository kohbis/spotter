use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::error::Error;
use std::fmt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// AWS region (default: us-east-1)
    #[arg(short, long, default_value = "us-east-1")]
    pub region: String,

    /// EC2 instance type to filter by (family like 'm5', size like 'large', or full type like 'm5.large')
    #[arg(short, long)]
    pub instance_type: Option<String>,

    /// Show spot prices for Linux and Windows (for latest pricing information, check AWS Management Console)
    #[arg(long)]
    pub spot_price: bool,

    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

#[derive(Debug)]
pub struct InvalidRegionError {
    pub region: String,
}

impl fmt::Display for InvalidRegionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid AWS region '{}'. Please use a valid AWS region code (e.g., us-east-1, eu-west-1, ap-northeast-1)",
            self.region
        )
    }
}

impl Error for InvalidRegionError {}

/// List of valid AWS regions
/// ref: https://docs.aws.amazon.com/global-infrastructure/latest/regions/aws-regions.html
const VALID_AWS_REGIONS: &[&str] = &[
    "us-east-1",      // US East (N. Virginia)
    "us-east-2",      // US East (Ohio)
    "us-west-1",      // US West (N. California)
    "us-west-2",      // US West (Oregon)
    "af-south-1",     // Africa (Cape Town)
    "ap-east-1",      // Asia Pacific (Hong Kong)
    "ap-south-1",     // Asia Pacific (Mumbai)
    "ap-south-2",     // Asia Pacific (Hyderabad)
    "ap-southeast-1", // Asia Pacific (Singapore)
    "ap-southeast-2", // Asia Pacific (Sydney)
    "ap-southeast-3", // Asia Pacific (Jakarta)
    "ap-southeast-4", // Asia Pacific (Melbourne)
    "ap-northeast-1", // Asia Pacific (Tokyo)
    "ap-northeast-2", // Asia Pacific (Seoul)
    "ap-northeast-3", // Asia Pacific (Osaka)
    "ca-central-1",   // Canada (Central)
    "ca-west-1",      // Canada (Calgary)
    "eu-central-1",   // Europe (Frankfurt)
    "eu-central-2",   // Europe (Zurich)
    "eu-west-1",      // Europe (Ireland)
    "eu-west-2",      // Europe (London)
    "eu-west-3",      // Europe (Paris)
    "eu-south-1",     // Europe (Milan)
    "eu-south-2",     // Europe (Spain)
    "eu-north-1",     // Europe (Stockholm)
    "il-central-1",   // Israel (Tel Aviv)
    "me-south-1",     // Middle East (Bahrain)
    "me-central-1",   // Middle East (UAE)
    "sa-east-1",      // South America (São Paulo)
];

/// Validates if the provided region is a valid AWS region
pub fn validate_region(region: &str) -> Result<(), InvalidRegionError> {
    if VALID_AWS_REGIONS.contains(&region) {
        Ok(())
    } else {
        Err(InvalidRegionError {
            region: region.to_string(),
        })
    }
}

impl Cli {
    /// Validates the CLI arguments
    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        validate_region(&self.region)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_default_values() {
        let cli = Cli::parse_from(["spotter"]);
        assert_eq!(cli.region, "us-east-1");
        assert_eq!(cli.instance_type, None);
        assert_eq!(cli.spot_price, false);
    }

    #[test]
    fn test_cli_with_region() {
        let cli = Cli::parse_from(["spotter", "--region", "eu-west-1"]);
        assert_eq!(cli.region, "eu-west-1");
        assert_eq!(cli.instance_type, None);
        assert_eq!(cli.spot_price, false);

        // Test short form
        let cli = Cli::parse_from(["spotter", "-r", "ap-northeast-1"]);
        assert_eq!(cli.region, "ap-northeast-1");
    }

    #[test]
    fn test_cli_with_instance_type() {
        let cli = Cli::parse_from(["spotter", "--instance-type", "m5.large"]);
        assert_eq!(cli.region, "us-east-1");
        assert_eq!(cli.instance_type, Some("m5.large".to_string()));
        assert_eq!(cli.spot_price, false);

        // Test short form
        let cli = Cli::parse_from(["spotter", "-i", "t3"]);
        assert_eq!(cli.instance_type, Some("t3".to_string()));
    }

    #[test]
    fn test_cli_with_spot_price() {
        let cli = Cli::parse_from(["spotter", "--spot-price"]);
        assert_eq!(cli.region, "us-east-1");
        assert_eq!(cli.instance_type, None);
        assert_eq!(cli.spot_price, true);
    }

    #[test]
    fn test_cli_with_multiple_args() {
        let cli = Cli::parse_from([
            "spotter",
            "--region",
            "us-west-2",
            "--instance-type",
            "c5.xlarge",
            "--spot-price",
        ]);
        assert_eq!(cli.region, "us-west-2");
        assert_eq!(cli.instance_type, Some("c5.xlarge".to_string()));
        assert_eq!(cli.spot_price, true);
    }

    #[test]
    fn test_cli_help() {
        let mut cmd = Cli::command();
        cmd.build();

        // This test just verifies that the help command can be built without errors
        // We're not checking the actual help text content
    }

    #[test]
    fn test_validate_region_valid() {
        // Test valid regions
        assert!(validate_region("us-east-1").is_ok());
        assert!(validate_region("eu-west-1").is_ok());
        assert!(validate_region("ap-northeast-1").is_ok());
        assert!(validate_region("ca-central-1").is_ok());
        assert!(validate_region("sa-east-1").is_ok());
    }

    #[test]
    fn test_validate_region_invalid() {
        // Test invalid regions
        let result = validate_region("invalid-region");
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.region, "invalid-region");
        assert!(
            error
                .to_string()
                .contains("Invalid AWS region 'invalid-region'")
        );

        // Test other invalid regions
        assert!(validate_region("us-east-3").is_err());
        assert!(validate_region("eu-west-4").is_err());
        assert!(validate_region("").is_err());
        assert!(validate_region("not-a-region").is_err());
    }

    #[test]
    fn test_cli_validate_valid_region() {
        let cli = Cli::parse_from(["spotter", "--region", "us-west-2"]);
        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_cli_validate_invalid_region() {
        let cli = Cli::parse_from(["spotter", "--region", "invalid-region"]);
        let result = cli.validate();
        assert!(result.is_err());

        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("Invalid AWS region 'invalid-region'"));
    }

    #[test]
    fn test_cli_validate_default_region() {
        let cli = Cli::parse_from(["spotter"]);
        assert!(cli.validate().is_ok());
        assert_eq!(cli.region, "us-east-1");
    }
}
