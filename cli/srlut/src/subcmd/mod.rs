use clap::Subcommand;

use self::{extend::ExtendArgs, gen::GenArgs, init::InitArgs};

mod extend;
mod gen;
mod init;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Gen(GenArgs),
    Extend(ExtendArgs),
    Init(InitArgs),
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match args.subcmd {
            Self::Gen(_) => GenArgs::run(args).await,
            Self::Extend(_) => ExtendArgs::run(args).await,
            Self::Init(_) => InitArgs::run(args).await,
        }
    }
}
