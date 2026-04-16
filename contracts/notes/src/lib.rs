#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, Symbol, Vec, String, Address
};

#[contracttype]
#[derive(Clone)]
pub struct Account {
    pub owner: Address,
    pub balance: i128,
}

#[contracttype]
#[derive(Clone)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
    pub note: String,
}

// Storage keys
const ACCOUNTS: Symbol = symbol_short!("ACCOUNTS");
const TXS: Symbol = symbol_short!("TXS");

#[contract]
pub struct BankContract;

#[contractimpl]
impl BankContract {

    // ========================
    // CREATE ACCOUNT
    // ========================
    pub fn create_account(env: Env, user: Address) {
        let mut accounts: Vec<Account> =
            env.storage().instance().get(&ACCOUNTS)
            .unwrap_or(Vec::new(&env));

        // cek kalau sudah ada
        for acc in accounts.iter() {
            if acc.owner == user {
                panic!("Account sudah ada");
            }
        }

        let acc = Account {
            owner: user,
            balance: 0,
        };

        accounts.push_back(acc);
        env.storage().instance().set(&ACCOUNTS, &accounts);
    }

    // ========================
    // DEPOSIT
    // ========================
    pub fn deposit(env: Env, user: Address, amount: i128) {
        let mut accounts: Vec<Account> =
            env.storage().instance().get(&ACCOUNTS)
            .unwrap_or(Vec::new(&env));

        for i in 0..accounts.len() {
            let mut acc = accounts.get(i).unwrap();
            if acc.owner == user {
                acc.balance += amount;
                accounts.set(i, acc);
                env.storage().instance().set(&ACCOUNTS, &accounts);
                return;
            }
        }

        panic!("Account tidak ditemukan");
    }

    // ========================
    // TRANSFER
    // ========================
    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
        note: String,
    ) {
        let mut accounts: Vec<Account> =
            env.storage().instance().get(&ACCOUNTS)
            .unwrap_or(Vec::new(&env));

        let mut found_from = false;
        let mut found_to = false;

        // update saldo
        for i in 0..accounts.len() {
            for i in 0..accounts.len() {
    let mut acc = accounts.get(i).unwrap();

    if acc.owner == from {
        if acc.balance < amount {
            panic!("Saldo tidak cukup");
        }
        acc.balance -= amount;
        accounts.set(i, acc);
        found_from = true;
    } else if acc.owner == to {
        acc.balance += amount;
        accounts.set(i, acc);
        found_to = true;
    }
}
        }

        if !found_from || !found_to {
            panic!("Account tidak ditemukan");
        }

        // simpan transaksi
        let mut txs: Vec<Transaction> =
            env.storage().instance().get(&TXS)
            .unwrap_or(Vec::new(&env));

        let tx = Transaction {
            from,
            to,
            amount,
            note,
        };

        txs.push_back(tx);
        env.storage().instance().set(&TXS, &txs);
    }

    // ========================
    // GET BALANCE
    // ========================
    pub fn get_balance(env: Env, user: Address) -> i128 {
        let accounts: Vec<Account> =
            env.storage().instance().get(&ACCOUNTS)
            .unwrap_or(Vec::new(&env));

        for acc in accounts.iter() {
            if acc.owner == user {
                return acc.balance;
            }
        }

        return 0;
    }

    // ========================
    // GET TRANSACTIONS
    // ========================
    pub fn get_transactions(env: Env) -> Vec<Transaction> {
        return env.storage().instance().get(&TXS)
            .unwrap_or(Vec::new(&env));
    }
}