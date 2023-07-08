use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ExcellarError {
    DepositMustBePositive = 1,
    WithdrawalMustBePositive = 2,
    InsufficientBalance = 3,
}
