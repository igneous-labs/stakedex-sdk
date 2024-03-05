use clap::Args;

#[derive(Args, Debug)]
#[clap(long_about = "Generate the json pubkey list of accounts that the LUT should contain")]
pub struct GenArgs;

impl GenArgs {
    pub async fn run(_args: crate::Args) {}
}
