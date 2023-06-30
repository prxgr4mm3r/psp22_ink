#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod psp22_ink {
    use ink::storage::Mapping;
    use ink::primitives::*;
    use ink::prelude::string::{String, ToString};

    #[ink(storage)]
    #[derive(Default)]
    pub struct Psp22Ink {
        total_supply: Balance,

        balances: Mapping<AccountId, Balance>,

        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum PSP22Error {
        /// Custom error type for cases in which an implementation adds its own restrictions.
        Custom(String),
        /// Returned if not enough balance to fulfill a request is available.
        InsufficientBalance,
        /// Returned if not enough allowance to fulfill a request is available.
        InsufficientAllowance,
        /// Returned if recipient's address is zero.
        ZeroRecipientAddress,
        /// Returned if sender's address is zero.
        ZeroSenderAddress,
        /// Returned if a safe transfer check fails (e.g. if the receiving contract does not accept tokens).
        SafeTransferCheckFailed(String),
    }

    pub type Result<T> = core::result::Result<T, PSP22Error>;

    #[ink::trait_definition]
    pub trait PSP22{
        #[ink(message)]
        fn total_supply(&self) -> Balance;

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance;

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance;

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()>;

        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()>;

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()>;

        #[ink(message)]
        fn increase_allowance(&mut self, spender: AccountId, added_value: Balance) -> Result<()>;

        #[ink(message)]
        fn decrease_allowance(&mut self, spender: AccountId, subtracted_value: Balance) -> Result<()>;
    }

    impl Psp22Ink {

        #[ink(constructor)]
        pub fn new(total_supply : Balance) -> Self {
            let mut balances = Mapping::default();
            balances.insert(Self::env().caller(), &total_supply);
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(Self::env().caller()),
                value: total_supply,
            });
            Self {
                balances,
                total_supply,
                allowances: Mapping::default(),
            }
        }
    }

    impl PSP22 for Psp22Ink{
        #[ink(message)]
        fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(&owner).unwrap_or_default()
        }
        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get(&(owner, spender)).unwrap_or_default()
        }
        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            //Reverts with error `InsufficientBalance` if there are not enough tokens on, the caller's account Balance.
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(PSP22Error::InsufficientBalance);
            }
            //Reverts with error `ZeroSenderAddress` if sender's address is zero.
            if from == AccountId::from([0x0; 32]) {
                return Err(PSP22Error::ZeroSenderAddress);
            }
            //Reverts with error `ZeroRecipientAddress` if recipient's address is zero.
            if to == AccountId::from([0x0; 32]) {
                return Err(PSP22Error::ZeroRecipientAddress);
            }
            //Reverts with error `SafeTransferCheckFailed` if the recipient is a contract and rejected the transfer.
            if self.env().is_contract(&to){
                return Err(PSP22Error::SafeTransferCheckFailed(
                    "Recipient is a contract and does not implement safe transfer behavior".to_string(),
                ));
            }
            //Decreases the balance of `from` and increases the balance of `to` by the same amount.
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            //Emits a `Transfer` event with `from` set to `None` if the sender is the zero address, otherwise to `Some(sender)`.
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            Ok(())

        }

        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);
            //Reverts with error `InsufficientAllowance` if there are not enough tokens allowed for the caller's account.
            if allowance < value {
                return Err(PSP22Error::InsufficientAllowance);
            }
            //Reverts with error `InsufficientBalance` if there are not enough tokens on the `from` account.
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(PSP22Error::InsufficientBalance);
            }
            //Reverts with error `ZeroSenderAddress` if sender's address is zero.
            if from == AccountId::from([0x0; 32]) {
                return Err(PSP22Error::ZeroSenderAddress);
            }
            //Reverts with error `ZeroRecipientAddress` if recipient's address is zero.
            if to == AccountId::from([0x0; 32]) {
                return Err(PSP22Error::ZeroRecipientAddress);
            }
            //Reverts with error `SafeTransferCheckFailed` if the recipient is a contract and rejected the transfer.
            if self.env().is_contract(&to) {
                return Err(PSP22Error::SafeTransferCheckFailed(
                    "Recipient is a contract and does not implement safe transfer behavior".to_string(),
                ));
            }
            //Decreases the allowance by the transferred amount.
            self.allowances.insert((from, caller), &(allowance - value));
            //Decreases the balance of `from` and increases the balance of `to` by the same amount.
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            //Emits a `Transfer` event with `from` set to `None` if the sender is the zero address, otherwise to `Some(sender)`.
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            Ok(())
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            //Sets `value` as the allowance of `spender` over the caller's tokens.
            self.allowances.insert((&owner, &spender), &value);
            //Emits an `Approval` event.
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }

        #[ink(message)]
        fn increase_allowance(&mut self, spender: AccountId, added_value: Balance) -> Result<()> {
            let owner = self.env().caller();
            //Increases the allowance of `spender` over the caller's tokens by `added_value`.
            let allowance = self.allowance(owner, spender);

            //Reverts with error `ZeroSenderAddress` if sender's address is zero.
            if owner == AccountId::from([0x0; 32]) {
                return Err(PSP22Error::ZeroSenderAddress);
            }
            //Reverts with error `ZeroRecipientAddress` if recipient's address is zero.
            if spender == AccountId::from([0x0; 32]) {
                return Err(PSP22Error::ZeroRecipientAddress);
            }
            self.allowances.insert((&owner, &spender), &(allowance + added_value));
            //Emits an `Approval` event.
            self.env().emit_event(Approval {
                owner,
                spender,
                value: allowance + added_value,
            });
            Ok(())
        }

        #[ink(message)]
        fn decrease_allowance(&mut self, spender: AccountId, subtracted_value: Balance) -> Result<()> {
            let owner = self.env().caller();
            //Decreases the allowance of `spender` over the caller's tokens by `subtracted_value`.
            let allowance = self.allowance(owner, spender);
            //Reverts with error `InsufficientAllowance` if there are not enough tokens allowed for the caller's account.
            if allowance < subtracted_value {
                return Err(PSP22Error::InsufficientAllowance);
            }
            //Reverts with error `ZeroSenderAddress` if sender's address is zero.
            if owner == AccountId::from([0x0; 32]) {
                return Err(PSP22Error::ZeroSenderAddress);
            }
            //Reverts with error `ZeroRecipientAddress` if recipient's address is zero.
            if spender == AccountId::from([0x0; 32]) {
                return Err(PSP22Error::ZeroRecipientAddress);
            }
            self.allowances.insert((&owner, &spender), &(allowance - subtracted_value));
            //Emits an `Approval` event.
            self.env().emit_event(Approval {
                owner,
                spender,
                value: allowance - subtracted_value,
            });
            Ok(())
        }
    }

}
