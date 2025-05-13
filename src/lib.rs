
use near_sdk::{env, near, store::LookupMap, AccountId};

#[near(contract_state)]
pub struct NearDIDRegistry {
    owners: LookupMap<AccountId, AccountId>,
    delegates: LookupMap<(AccountId, String, AccountId), u64>,
    attributes: LookupMap<(AccountId, String, Vec<u8>), u64>,
    changed: LookupMap<AccountId, u64>,
    nonce: LookupMap<AccountId, u64>,
}

impl Default for NearDIDRegistry {
    fn default() -> Self {
        Self {
            owners: LookupMap::new(b"o"),
            delegates: LookupMap::new(b"d"),
            attributes: LookupMap::new(b"a"),
            changed: LookupMap::new(b"c"),
            nonce: LookupMap::new(b"n"),
        }
    }
}

#[near]
impl NearDIDRegistry {
    fn assert_only_owner(&self, identity: &AccountId, actor: &AccountId) {
        let owner = self.identity_owner(identity.clone());
        assert_eq!(actor, &owner, "bad_actor");
    }

    pub fn identity_owner(&self, identity: AccountId) -> AccountId {
        self.owners.get(&identity).unwrap_or(&identity).clone()
    }

    pub fn change_owner(&mut self, identity: AccountId, new_owner: AccountId) {
        let actor = env::predecessor_account_id();
        self.assert_only_owner(&identity, &actor);

        self.owners.insert(identity.clone(), new_owner);
        self.changed.insert(identity, env::block_height());
    }

    pub fn add_delegate(&mut self, identity: AccountId, delegate_type: String, delegate: AccountId, validity_secs: u64) {
        let actor = env::predecessor_account_id();
        self.assert_only_owner(&identity, &actor);

        let valid_until = env::block_timestamp_ms() / 1000 + validity_secs;
        self.delegates.insert((identity.clone(), delegate_type.clone(), delegate.clone()), valid_until);
        self.changed.insert(identity, env::block_height());
    }

    pub fn revoke_delegate(&mut self, identity: AccountId, delegate_type: String, delegate: AccountId) {
        let actor = env::predecessor_account_id();
        self.assert_only_owner(&identity, &actor);

        self.delegates.insert((identity.clone(), delegate_type.clone(), delegate.clone()), 0);
        self.changed.insert(identity, env::block_height());
    }

    pub fn valid_delegate(&self, identity: AccountId, delegate_type: String, delegate: AccountId) -> bool {
        match self.delegates.get(&(identity, delegate_type, delegate)) {
            Some(valid_until) => valid_until > &(env::block_timestamp_ms() / 1000),
            None => false,
        }
    }

    pub fn set_attribute(&mut self, identity: AccountId, name: String, value: Vec<u8>, validity_secs: u64) {
        let actor = env::predecessor_account_id();
        self.assert_only_owner(&identity, &actor);

        let valid_until = env::block_timestamp_ms() / 1000 + validity_secs;
        self.attributes.insert((identity.clone(), name.clone(), value.clone()), valid_until);
        self.changed.insert(identity, env::block_height());
    }

    pub fn revoke_attribute(&mut self, identity: AccountId, name: String, value: Vec<u8>) {
        let actor = env::predecessor_account_id();
        self.assert_only_owner(&identity, &actor);

        self.attributes.insert((identity.clone(), name.clone(), value.clone()), 0);
        self.changed.insert(identity, env::block_height());
    }

    pub fn valid_attribute(&self, identity: AccountId, name: String, value: Vec<u8>) -> bool {
        match self.attributes.get(&(identity, name, value)) {
            Some(valid_until) => valid_until > &(env::block_timestamp_ms() / 1000),
            None => false,
        }
    }

    pub fn get_nonce(&self, identity: AccountId) -> u64 {
        *self.nonce.get(&identity).unwrap_or(&0)
    }

    pub fn increment_nonce(&mut self, identity: AccountId) {
        let n = self.nonce.get(&identity).unwrap_or(&0);
        self.nonce.insert(identity, n + 1);
    }

    pub fn get_changed(&self, identity: AccountId) -> u64 {
        *self.changed.get(&identity).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{test_utils::{accounts, VMContextBuilder}, testing_env};

    fn set_context(predecessor: AccountId) {
        let mut builder = VMContextBuilder::new();
        builder
            .predecessor_account_id(predecessor)
            .block_height(40)
            .block_timestamp(1_000_000)
            // .attached_deposit(0)
            // .account_balance(0)
            .is_view(false);

        testing_env!(builder.build());
    }

    #[test]
    fn identity_owner() {
        let owner = accounts(1);
        let contract = NearDIDRegistry::default();
        assert_eq!(contract.identity_owner(owner.clone()), owner);
    }

    #[test]
    fn change_owner_success() {
        let owner = accounts(1);
        let new_owner = accounts(2);
        set_context(owner.clone());

        let mut contract = NearDIDRegistry::default();

        assert_eq!(contract.identity_owner(owner.clone()), owner);

        contract.change_owner(owner.clone(), new_owner.clone());

        assert_eq!(contract.identity_owner(owner.clone()), new_owner);
        // assert_eq!(contract.get_changed(owner), 40);
    }

    #[test]
    #[should_panic(expected = "bad_actor")]
    fn change_owner_fails() {
        let owner = accounts(1);
        let attacker = accounts(3);
        let new_owner = accounts(2);

        set_context(attacker.clone());

        let mut contract = NearDIDRegistry::default();
        contract.change_owner(owner, new_owner);
    }

    #[test]
    fn add_delegate_success() {
        let identity = accounts(1);
        let owner = identity.clone();
        let delegate = accounts(2);
        let delegate_type = "veriKey".to_string();
        let validity_secs = 3600;

        set_context(owner.clone());

        let mut contract = NearDIDRegistry::default();
        contract.add_delegate(identity.clone(), delegate_type.clone(), delegate.clone(), validity_secs);

        let valid = contract.valid_delegate(identity.clone(), delegate_type.clone(), delegate.clone());
        assert!(valid, "El delegado debería ser válido");
    }

    #[test]
    #[should_panic(expected = "bad_actor")]
    fn add_delegate_fails() {
        let identity = accounts(1);
        let attacker = accounts(3);
        let delegate = accounts(2);
        let delegate_type = "veriKey".to_string();

        set_context(attacker.clone());

        let mut contract = NearDIDRegistry::default();
        contract.add_delegate(identity, delegate_type, delegate, 1000);
    }

    #[test]
    fn revoke_delegate_success() {
        let identity = accounts(1);
        let owner = identity.clone();
        let delegate = accounts(2);
        let delegate_type = "veriKey".to_string();
        let validity_secs = 3600;

        set_context(owner.clone());

        let mut contract = NearDIDRegistry::default();

        contract.add_delegate(identity.clone(), delegate_type.clone(), delegate.clone(), validity_secs);
        assert!(contract.valid_delegate(identity.clone(), delegate_type.clone(), delegate.clone()));

        contract.revoke_delegate(identity.clone(), delegate_type.clone(), delegate.clone());
        assert!(!contract.valid_delegate(identity.clone(), delegate_type.clone(), delegate.clone()));
    }

    #[test]
    #[should_panic(expected = "bad_actor")]
    fn revoke_delegate_fails() {
        let identity = accounts(1);
        let attacker = accounts(3);
        let delegate = accounts(2);
        let delegate_type = "veriKey".to_string();

        set_context(attacker.clone());

        let mut contract = NearDIDRegistry::default();

        contract.revoke_delegate(identity, delegate_type, delegate);
    }

    #[test]
    fn set_attribute_success() {
        let identity = accounts(1);
        let owner = identity.clone();
        let name = "did/pub/Ed25519/veriKey/base64".to_string();
        let value = b"base64EncodedKeyHere".to_vec();
        let validity_secs = 3600;

        set_context(owner.clone());

        let mut contract = NearDIDRegistry::default();

        contract.set_attribute(identity.clone(), name.clone(), value.clone(), validity_secs);

        let stored_valid_until = contract
            .attributes
            .get(&(identity.clone(), name.clone(), value.clone()))
            .unwrap();
        assert_eq!(stored_valid_until, &validity_secs);
    }

    #[test]
    #[should_panic(expected = "bad_actor")]
    fn test_set_attribute_fails_if_not_owner() {
        let identity = accounts(1);
        let attacker = accounts(3);
        let name = "did/pub/Ed25519/veriKey/base64".to_string();
        let value = b"maliciousKey".to_vec();

        set_context(attacker.clone());

        let mut contract = NearDIDRegistry::default();

        contract.set_attribute(identity, name, value, 1000);
    }

    #[test]
    fn test_revoke_attribute_success() {
        let identity = accounts(1);
        let owner = identity.clone();
        let name = "did/service/endpoint".to_string();
        let value = b"https://example.com".to_vec();
        let validity_secs = 3600;

        set_context(owner.clone());

        let mut contract = NearDIDRegistry::default();

        contract.set_attribute(identity.clone(), name.clone(), value.clone(), validity_secs);
        assert!(contract.valid_attribute(identity.clone(), name.clone(), value.clone()));

        contract.revoke_attribute(identity.clone(), name.clone(), value.clone());
        assert!(!contract.valid_attribute(identity.clone(), name.clone(), value.clone()));

        let stored = contract
            .attributes
            .get(&(identity.clone(), name.clone(), value.clone()))
            .unwrap();
        assert_eq!(stored, &0, "El atributo debe estar revocado (valor 0)");
    }
}
