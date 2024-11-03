use crate::*;
use alloy::{
    eips::BlockNumberOrTag,
    primitives::{Address, U256},
    rpc::types::Log,
    sol_types::SolEvent,
};

pub fn build_locked_liquidity_filter() -> Filter {
    let events = LogDecoder::LogDecoderEvents::SELECTORS
        .iter()
        .map(|e| e.into())
        .collect::<Vec<_>>();

    Filter::new()
        .select(BlockNumberOrTag::Latest)
        .event_signature(events)
}

pub fn decode_it(log: &Log) -> Option<(Address, U256)> {
    // Decode the log
    match *log.topic0()? {
        // UNCX V2
        LogDecoder::onDeposit::SIGNATURE_HASH => {
            if let Ok(data) = LogDecoder::onDeposit::decode_log(&log.inner, false) {
                return Some((data.lpToken, data.unlockDate));
            }
        }
        // UNCX V3
        LogDecoder::onLock::SIGNATURE_HASH => {
            if let Ok(data) = LogDecoder::onLock::decode_log(&log.inner, false) {
                return Some((data.poolAddress, data.unlockDate));
            }
        }
        _ => return None,
    }

    None
}
