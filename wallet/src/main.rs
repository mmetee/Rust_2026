enum TransactionStatus {
    Success,
    Failed(String),
    Pending,
}
struct Wallet {
    owner: String,
    balance: f64,
}
impl Wallet {
    fn new(name: &str) -> Wallet {
        Wallet {
            owner: name.to_string(),
            balance: 0.0,
        }
    }
    fn display_balance(&self) {
        println!("คุณ {} มียอดเงินคงเหลือ {} บาท", self.owner, self.balance);
    }
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("ฝากเงินสำเร็จ +{}", amount);
    }
    fn withdraw(&mut self, amount: f64) -> TransactionStatus {
        if amount > self.balance {
            TransactionStatus::Failed("ยอดเงินไม่เพียงพอ".to_string())
        } else {
            self.balance -= amount;
            TransactionStatus::Success
        }
    }
}
fn main() {
    let mut my_wallet = Wallet::new("Metee");
    my_wallet.deposit(500.0);
    my_wallet.withdraw(200.0);
    my_wallet.display_balance();

    let result = my_wallet.withdraw(1000.00);
    match result {
        TransactionStatus::Success => println!("ถอนเงินสำเร็จ"),
        TransactionStatus::Failed(reason) => println!("ถอนเงินไม่ได้เพราะ :{}", reason),
        TransactionStatus::Pending => println!("กรุณารอสักครู่..."),
    }
}
