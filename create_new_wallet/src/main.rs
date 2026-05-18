struct Wallet {
    public_address: String,
    private_key: String,
    bitcoin_balance: f64,
    status: String,
}

impl Wallet {
    fn generate(public_address: String, private_key: String, bitcoin_balance: f64) -> Wallet {
        Wallet {
            public_address,
            private_key,
            bitcoin_balance,
            status: String::from("Active"),
        }
    }
    fn check_balance(&self) {
        println!("wallet  public addres Name: {}", self.public_address);
        println!(" wallet private address: {}", self.private_key);
        println!(" bitcoin balance: {}", self.bitcoin_balance);
        println!(" wallet status: {}", self.status);
    }
    fn recieve_new_bitcoin(&mut self, amount: f64) {
        println!(" new balance Add: {}", amount);
        println!("loading.....");
        self.bitcoin_balance += amount;
    }
    fn send_bitcoin(&mut self, qty: f64) {
        if self.bitcoin_balance >= qty {
            self.bitcoin_balance -= qty;
            println!("تم الإرسال بنجاح!");
        } else {
            println!("⚠️ رصيدك غير كافٍ لإتمام هذه المعاملة!");
        }
    }
}

fn main() {
    let mut new_wallet = Wallet::generate(
        String::from("2cf24dba4f21d4288094c30e2ede82c380cac19544bb5c4ab02f5b2db38500d3"),
        String::from("185f8db32271fe25f561a6fc938b2e264306ec304eda518007d1764826381969"),
        0.0,
    );
    println!("\n ---  new wallet info  ---");
    new_wallet.check_balance();
    println!("\n ---  new bitcoin add  ---");
    new_wallet.recieve_new_bitcoin(200.0);
    new_wallet.check_balance();
    println!("\n");
    new_wallet.send_bitcoin(100.0);
    new_wallet.check_balance();
}
