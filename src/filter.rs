use crate::*;
use alloy::{eips::BlockNumberOrTag, primitives::Address, rpc::types::Log, sol_types::SolEvent};

pub fn build_locked_liquidity_filter() -> Filter {
    Filter::new().select(BlockNumberOrTag::Latest)
}

pub fn decode_it(log: &Log) -> Option<Address> {
    // Decode the log
    match *log.topic0()? {
        LogDecoder::onDeposit::SIGNATURE_HASH => {
            if let Ok(data) = LogDecoder::onDeposit::decode_log(&log.inner, false) {
                return Some(data.lpToken);
            }
        }
        _ => return None,
    }

    None
}
