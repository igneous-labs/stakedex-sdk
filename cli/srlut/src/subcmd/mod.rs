use clap::Subcommand;

use self::{extend::ExtendArgs, gen::GenArgs};

mod extend;
mod gen;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Gen(GenArgs),
    Extend(ExtendArgs),
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match args.subcmd {
            Self::Gen(_) => GenArgs::run(args).await,
            Self::Extend(_) => ExtendArgs::run(args).await,
        }
    }
}
