#[allow(dead_code)]
#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn add_account(bank: &mut Bank, account: Account) {
    bank.accounts.push(account);
}

fn main() {
    let mut bank = Bank::new();

    let account = Account::new(1, String::from("Manu"));

    add_account(&mut bank, account);

    println!("Bank: {bank:#?}");
}
