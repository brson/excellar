use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum LendCraftError {
    DepositMustBePositive = 1,
    WithdrawalMustBePositive = 2,
    InsufficientBalance = 3,
}
