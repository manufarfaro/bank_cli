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

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;

        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;

        self.balance
    }

    fn summary(&self) -> String {
        format!(
            "{}: {} has a balance of {}",
            self.id, self.holder, self.balance
        )
    }
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    let mut bank = Bank::new();

    let account = Account::new(1, String::from("Manu"));

    bank.add_account(account);

    bank.accounts[0].deposit(123);
    bank.accounts[0].withdraw(33);
    bank.accounts[0].deposit(123);

    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
}
