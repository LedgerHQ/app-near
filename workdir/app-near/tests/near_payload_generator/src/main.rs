use std::str::FromStr;

use near_crypto::{ED25519SecretKey, InMemorySigner, KeyType, PublicKey, SecretKey};
use near_primitives::account::id::AccountId;
use near_primitives::account::{AccessKey, AccessKeyPermission, FunctionCallPermission};
use near_primitives::borsh::BorshSerialize;
use near_primitives::hash::CryptoHash;
use near_primitives::transaction::{
    Action, AddKeyAction, DeleteAccountAction, DeleteKeyAction, FunctionCallAction,
    SignedTransaction, StakeAction, Transaction, TransferAction,
};

use anyhow::{Error, Result};

fn print_hex(data: &[u8]) {
    for el in data.into_iter() {
        print!("{:02x}", el);
    }
    print!("\n");
}

fn print_test_case_info(signed_transaction: SignedTransaction) {
    println!("{:#?}", &signed_transaction.transaction);
    println!("\nNear payload:");
    print_hex(&signed_transaction.transaction.try_to_vec().unwrap());
    println!("\nExpected signature:");
    // Remove leading 0
    print_hex(&signed_transaction.signature.try_to_vec().unwrap()[1..]);
}

//  "glory promote mansion idle axis finger extra february uncover one trip resource lawn turtle enact monster seven myth punch hobby comfort wild raise skin";

const ED25519_KEYPAIR: &str =
    "188d2ce61071d477a2400558c3612ee68957a80aa2e56c29dc4da2dace58e7d8c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f";

fn prepare_signer() -> Result<InMemorySigner> {
    // Prepare private key
    let raw = hex::decode(ED25519_KEYPAIR)?;
    let key_array = raw.try_into().unwrap();
    let ed25519_private_key = SecretKey::ED25519(ED25519SecretKey(key_array));

    let account_id = AccountId::from_str("dummy.near")?; // Dummy account id
    let signer = InMemorySigner::from_secret_key(account_id, ed25519_private_key);

    // Print key info
    // println!("Speculos key: {:?}", signer.public_key.key_data());
    println!("Speculos key:");
    print_hex(signer.public_key.key_data());

    Ok(signer)
}

fn main() -> Result<(), Error> {
    // pub enum Action {
    //     /// Create an (sub)account using a transaction `receiver_id` as an ID for
    //     /// a new account ID must pass validation rules described here
    //     /// <http://nomicon.io/Primitives/Account.html>.
    //     CreateAccount(CreateAccountAction),
    //     /// Sets a Wasm code to a receiver_id
    //     DeployContract(DeployContractAction),
    //     FunctionCall(FunctionCallAction),
    //     Transfer(TransferAction),
    //     Stake(StakeAction),
    //     AddKey(AddKeyAction),
    //     DeleteKey(DeleteKeyAction),
    //     DeleteAccount(DeleteAccountAction),
    // }

    let signer = prepare_signer()?;

    let transactions = vec![
        // Test tranfer transaction
        Transaction {
            signer_id: AccountId::from_str("blablatest.testnet")?,
            public_key: PublicKey::from_str("ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U")
                .unwrap(),
            nonce: 96520360000015,
            receiver_id: AccountId::from_str("speculos.testnet")?,
            block_hash: CryptoHash::from_str("C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK")
                .unwrap(),
            actions: vec![Action::Transfer(TransferAction {
                deposit: 123400000000000000000000,
            })],
        },
        // Function call
        Transaction {
            signer_id: AccountId::from_str("blablatest.testnet")?,
            public_key: PublicKey::from_str("ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U")
                .unwrap(),
            nonce: 96520360000015,
            receiver_id: AccountId::from_str("speculos.testnet")?,
            block_hash: CryptoHash::from_str("C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK")
                .unwrap(),
            actions: vec![Action::FunctionCall(FunctionCallAction {
                method_name: String::from("function_name"),
                args: vec![0xAA, 0xBB],
                gas: 9999,
                deposit: 1122334455,
            })],
        },
        // Stake
        Transaction {
            signer_id: AccountId::from_str("signer.near")?,
            public_key: PublicKey::from_str("ed25519:4c2pNM4aqrdTgaeRQyJnP9UwAFvHstDzZ1SCQAB7HnEc")
                .unwrap(),
            nonce: 96520360000015,
            receiver_id: AccountId::from_str("receiver.near")?,
            block_hash: CryptoHash::from_str("C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK")
                .unwrap(),
            actions: vec![Action::Stake(StakeAction {
                stake: 55556666,
                public_key: PublicKey::from_str(
                    "ed25519:J6DmXMFwt894ZXED1BCGiK3y1aRhophVob5VwL8JBTm1",
                )?,
            })],
        },
        // Add key: Full access
        Transaction {
            signer_id: AccountId::from_str("arthur")?,
            public_key: PublicKey::from_str("ed25519:JCuJVU1tbr2tmYGX8b6f3YpvuN2zBZd2MZAYh16cNqGr")
                .unwrap(),
            nonce: 96520360000015,
            receiver_id: AccountId::from_str(
                "98793cd91a3f870fb126f66285808c7e094afcfc4eda8a970f6648cdf0dbd6de",
            )?,
            block_hash: CryptoHash::from_str("C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK")
                .unwrap(),
            actions: vec![Action::AddKey(AddKeyAction {
                public_key: PublicKey::from_str(
                    "ed25519:79QAhRR464JQGr5ZXZe6jvXGM8NNgnR5KzRoQhaJyTYT",
                )?,
                access_key: AccessKey {
                    nonce: 12345,
                    permission: AccessKeyPermission::FullAccess,
                },
            })],
        },
        // Delete key
        Transaction {
            signer_id: AccountId::from_str("speculosaccount")?,
            public_key: PublicKey::from_str("ed25519:JCuJVU1tbr2tmYGX8b6f3YpvuN2zBZd2MZAYh16cNqGr")
                .unwrap(),
            nonce: 96520360000015,
            receiver_id: AccountId::from_str(
                "98793cd91a3f870fb126f66285808c7e094afcfc4eda8a970f6648cdf0dbd6de",
            )?,
            block_hash: CryptoHash::from_str("C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK")
                .unwrap(),
            actions: vec![Action::DeleteKey(DeleteKeyAction {
                public_key: PublicKey::from_str(
                    "ed25519:79QAhRR464JQGr5ZXZe6jvXGM8NNgnR5KzRoQhaJyTYT",
                )
                .unwrap(),
            })],
        },
        // Delete account
        Transaction {
            signer_id: AccountId::from_str("speculosaccount")?,
            public_key: PublicKey::from_str("ed25519:7aE719urLcxUn81B9RkvXwDTgnXM7DEAy1eGWU2nDNd9")
                .unwrap(),
            nonce: 96520360000015,
            receiver_id: AccountId::from_str("receiver.near")?,
            block_hash: CryptoHash::from_str("C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK")
                .unwrap(),
            actions: vec![Action::DeleteAccount(DeleteAccountAction {
                beneficiary_id: AccountId::from_str("beneficiaryid")?,
            })],
        },
        // Add key: Full access
        Transaction {
            signer_id: AccountId::from_str(
                "b282b6f9a571a27d3ccde60f7fe194cdd498da775342a1587c662c44bf214fb2",
            )?,
            public_key: PublicKey::from_str("ed25519:8MufT2vbLVH1tQxvZxNTbg1pamM3F6qSMv14X53wmdtC")
                .unwrap(),
            nonce: 96520360000015,
            receiver_id: AccountId::from_str(
                "98793cd91a3f870fb126f66285808c7e094afcfc4eda8a970f6648cdf0dbd6de",
            )?,
            block_hash: CryptoHash::from_str("C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK")
                .unwrap(),
            actions: vec![Action::AddKey(AddKeyAction {
                public_key: PublicKey::from_str(
                    "ed25519:79QAhRR464JQGr5ZXZe6jvXGM8NNgnR5KzRoQhaJyTYT",
                )?,
                access_key: AccessKey {
                    nonce: 12345,
                    permission: AccessKeyPermission::FunctionCall(FunctionCallPermission {
                        allowance: Some(9999999999),
                        receiver_id: String::from("Receiver id"),
                        method_names: vec![
                            String::from("Method 0"),
                            String::from("Method 1"),
                        ]
                    }),
                },
            })],
        },
    ];

    // Generate a public key
    for _ in 0..2 {
        let pb = SecretKey::from_random(KeyType::ED25519);
        println!("{}", pb.public_key());
    }

    for (i, transaction) in transactions.into_iter().enumerate() {
        println!("\n\n----- Test case {} -----", i);
        let signed_transaction = transaction.sign(&signer);
        print_test_case_info(signed_transaction);
    }
    Ok(())
}
