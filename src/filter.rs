use crate::*;
use alloy::eips::BlockNumberOrTag;

pub fn build_locked_liquidity_filter() -> Filter {
    Filter::new()
        .from_block(BlockNumberOrTag::Latest)
        .to_block(BlockNumberOrTag::Latest)
}
