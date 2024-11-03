use alloy::sol;

sol!(
    contract LogDecoder {
        // UNCX V2
        event onDeposit(address lpToken, address user, uint256 amount, uint256 lockDate, uint256 unlockDate);
    }
);
