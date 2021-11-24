use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
near_sdk::setup_alloc!();
use near_sdk::collections::{UnorderedMap, UnorderedSet};

pub trait NEP4 {
    fn grant_access(&mut self, escrow_account_id: AccountId);

    fn revoke_access(&mut self, escrow_account_id: AccountId);

    fn transfer_from(&mut self, owner_id: AccountId, new_owner_id: AccountId, token_id: TokenId);

    fn transfer(&mut self, new_owner_id: AccountId, token_id: TokenId);

    // Returns `true` or `false` based on caller of the function (`predecessor_id) having access to the token

    fn check_access(&self, account_id: AccountId) -> bool;

    // Get an individual owner by given `tokenId`.

    fn get_token_owner(&self, token_id: TokenId) -> String;
}

pub type TokenId = u64;
pub type AccountIdHash = Vec<u8>;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NonFungibleTokenBasic {
    pub token_to_account: UnorderedMap<TokenId, AccountId>,

    pub account_gives_access: UnorderedMap<AccountIdHash, AccountId>,
}

#[near_bindgen]

impl Default for NonFungibleTokenBasic {
    fn default() -> Self {
        Self {
            token_to_account: UnorderedMap::new(b"tta".to_vec()),

            account_gives_access: UnorderedMap::new(b"aga".to_vec()),
        }
    }
}

#[near_bindgen]

impl NonFungibleTokenBasic {
    pub fn mint_token(&mut self, owner_id: String, token_id: TokenId) {
        let token_check = self.token_to_account.get(&token_id);

        if token_check.is_some() {
            env::panic(b"Token ID already exists.")
        }

        self.token_to_account.insert(&token_id, &owner_id);
    }
}

#[near_bindgen]
impl NEP4 for NonFungibleTokenBasic {
    fn grant_access(&mut self, escrow_account_id: AccountId) {
        let escrow_hash = env::sha256(escrow_account_id.as_bytes());

        let predecessor = env::predecessor_account_id();

        let predecessor_hash = env::sha256(predecessor.as_bytes());

        let mut access_set = match self.account_gives_access.get(&predecessor_hash) {
            Some(existing_set) => existing_set,

            None => UnorderedSet::new(b"new-access-set".to_vec()),
        };

        access_set.insert(&escrow_hash);

        self.account_gives_access
            .insert(&predecessor_hash, &access_set);
    }

    fn revoke_access(&mut self, escrow_account_id: AccountId) {
        let predecessor = env::predecessor_account_id();

        let predecessor_hash = env::sha256(predecessor.as_bytes());

        let mut existing_set = match self.account_gives_access.get(&predecessor_hash) {
            Some(existing_set) => existing_set,

            None => env::panic(b"Access does not exist."),
        };

        let escrow_hash = env::sha256(escrow_account_id.as_bytes());

        if existing_set.contains(&escrow_hash) {
            existing_set.remove(&escrow_hash);

            self.account_gives_access
                .insert(&predecessor_hash, &existing_set);

            env::log(b"Successfully removed access.")
        } else {
            env::panic(b"Did not find access for escrow ID.")
        }
    }

    fn transfer(&mut self, new_owner_id: AccountId, token_id: TokenId) {
        let token_owner_account_id = self.get_token_owner(token_id);

        let predecessor = env::predecessor_account_id();

        if predecessor != token_owner_account_id {
            env::panic(b"Attempt to call transfer on tokens belonging to another account.")
        }

        self.token_to_account.insert(&token_id, &new_owner_id);
    }

    fn check_access(&self, account_id: AccountId) -> bool {
        let account_hash = env::sha256(account_id.as_bytes());

        let predecessor = env::predecessor_account_id();

        if predecessor == account_id {
            return true;
        }

        match self.account_gives_access.get(&account_hash) {
            Some(access) => {
                let predecessor = env::predecessor_account_id();

                let predecessor_hash = env::sha256(predecessor.as_bytes());

                access.contains(&predecessor_hash)
            }

            None => false,
        }
    }

    fn get_token_owner(&self, token_id: TokenId) -> String {
        match self.token_to_account.get(&token_id) {
            Some(owner_id) => owner_id,

            None => env::panic(b"No owner of the token ID specified"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
