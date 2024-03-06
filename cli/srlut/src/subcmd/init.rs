use clap::Args;
use sanctum_solana_cli_utils::{parse_signer, TxSendingNonblockingRpcClient};
use solana_sdk::{
    address_lookup_table::instruction::create_lookup_table,
    epoch_info::EpochInfo,
    message::{v0::Message, VersionedMessage},
    transaction::VersionedTransaction,
};

use super::Subcmd;

const RECENT_SLOT_PAST_BUFFER: u64 = 10;

#[derive(Args, Debug)]
#[clap(long_about = "Initializes the LUT")]
pub struct InitArgs {
    #[clap(
        long,
        short,
        help = "The LUT's admin. Defaults to config wallet if not set."
    )]
    pub auth: Option<String>,
}

impl InitArgs {
    pub async fn run(args: crate::Args) {
        let Self { auth } = match args.subcmd {
            Subcmd::Init(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();

        let auth_signer = auth.map(|s| parse_signer(&s).unwrap());
        let auth = auth_signer.as_ref().unwrap_or(&payer);

        let EpochInfo { absolute_slot, .. } = rpc.get_epoch_info().await.unwrap();
        let (ix, lut_addr) = create_lookup_table(
            auth.pubkey(),
            payer.pubkey(),
            absolute_slot - RECENT_SLOT_PAST_BUFFER,
        );

        let mut signers = vec![payer.as_ref(), auth.as_ref()];
        signers.dedup();

        let rbh = rpc.get_latest_blockhash().await.unwrap();
        let tx = VersionedTransaction::try_new(
            VersionedMessage::V0(Message::try_compile(&payer.pubkey(), &[ix], &[], rbh).unwrap()),
            &signers,
        )
        .unwrap();

        eprintln!("Creating LUT {lut_addr}");

        rpc.handle_tx(&tx, args.send_mode).await;
    }
}
