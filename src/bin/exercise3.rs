struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // associated function (constructor)
    fn new(owner: String, balance: f64) -> BankAccount {
        BankAccount { owner, balance }
    }

    // deposit money
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    // withdraw money
    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        } else {
            println!("Insufficient funds or invalid amount");
        }
    }

    // check balance (read-only)
    fn check_balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account = BankAccount::new(String::from("Flourish"), 1_000.0);

    account.deposit(500.0);
    account.withdraw(300.0);

    println!("Balance: {}", account.check_balance());
}
