
use ink_lang as ink;

#[int::contract]
mod erc20_token {
    use ink_prelude::vec::Vec;
    #[ink(storage)]
    pub struct Erc20Token {
        total_supply: Balance,
        balances: ink_storage::collections::HashMap<AccountId, Balance>,
        allowances: ink_storage::collections::HashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer{
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }
    #[ink(event)]
    pub struct Approval {
        owner: AccountId,
        spender: AccountId,
        value: Balance,
    }

    impl Erc20Token {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = ink_storage::collections::HashMap::new();
            balances.insert(caller, initial_supply);

            Self {
                total_supply: initial_supply,
                balances,
                allowances: ink_storage::collections::HashMap::new(),
                
            }
        }
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }
        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Balance {
            *self.balance.get(&account).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            let from = self.env().caller();
            self.transfer_internal(from, to, value)
        }
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> bool {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), value);

            self.env().emit_event(Approval{
                owner,
                spender,
                value,
            });
            true
        }
        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool{
            let caller = self.env().caller();
            let allowance = *self.allowances.get(&(from, caller)).unwrap_or(&0);

            if allowance < value || !self.transfer_internal(from, to, value) {
                return false;
            }

            self.allowances.insert((from, caller), allowance - value);
            true
        }
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            *self.allowances.get(&(owner, spender)).unwrap_or(&0)
        }
        fn transfer_internal(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool{
            let from_balance = *self.balances.get(&from).unwrap_or(&0);

            if from_balance < value || value == 0 {
                return false;
            }
            self.balances.insert(from, from_balance - value);

            let to_balance = *self.balances.get(&to).unwrap_or(&0);
            self.balances.insert(to, to_balance + value);

            self.env().emit_event(Transfer {
                from,
                to, 
                value,
            });
            true
        }
    }
    


}
