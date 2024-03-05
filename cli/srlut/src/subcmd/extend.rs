use clap::Args;

#[derive(Args, Debug)]
#[clap(long_about = "Extend the LUT with the accounts on a generated json pubkey list")]
pub struct ExtendArgs;

impl ExtendArgs {
    pub async fn run(_args: crate::Args) {}
}
