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
