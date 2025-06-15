use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

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
}
