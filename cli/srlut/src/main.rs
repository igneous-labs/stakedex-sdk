use clap::{builder::ValueParser, Parser};
use sanctum_solana_cli_utils::{ConfigWrapper, TxSendMode};
use subcmd::Subcmd;
use tokio::runtime::Runtime;

mod subcmd;

#[derive(Parser, Debug)]
#[clap(author, version, about = "Sanctum Router LUT Management CLI")]
pub struct Args {
    #[clap(
        long,
        short,
        help = "Path to solana CLI config. Defaults to ~/.config/solana/cli/config.yml.",
        default_value = "",
        value_parser = ValueParser::new(ConfigWrapper::parse_from_path)
    )]
    pub config: ConfigWrapper,

    #[clap(
        long,
        short,
        help = "Transaction send mode.
- send-actual: signs and sends the tx to the cluster specified in config and outputs hash to stderr
- sim-only: simulates the tx against the cluster and outputs logs to stderr
- dump-msg: dumps the base64 encoded tx to stdout. For use with inspectors and multisigs",
        default_value_t = TxSendMode::default(),
        value_enum,
    )]
    pub send_mode: TxSendMode,

    #[clap(subcommand)]
    pub subcmd: Subcmd,
}

fn main() {
    let args = Args::parse();
    let rt = Runtime::new().unwrap();
    rt.block_on(Subcmd::run(args));
}
