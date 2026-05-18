// struct BankAccount {
//     account_number: u32,
//     owner_name: String,
//     balance: f64,
//     status:String
// }
// fn main() {
//     let mut account = BankAccount {
//         account_number: 123,
//         owner_name: String::from("hazem hisham"),
//         balance: 500.0,
//         status:String::from("Active")
//     };

//     println!("\n ---   account info ---");
//     view_balance(&account);
//     println!("\n---   account add 200usd ---");
//     deposit(&mut account, 200.0);
//     println!("\n ---   new balance after deposit 200 ---");
//     view_balance(&account);

//     println!("\n ---   new balance after witdraw 300 ---");
//     withdraw(&mut account, 300.0);
//     view_balance(&account);

//     println!("\n ---close account status ---");
//     close_Account(&mut account,String::from("not active"));
//     view_balance(&account);
// }

// fn view_balance(account: &BankAccount) {
//     println!(" Owner Name: {}", account.owner_name);
//     println!(" accountnumber: {}", account.account_number);
//     println!(" balance: {}", account.balance);
//     println!(" balance: {}", account.status);
// }

// fn deposit(account: &mut BankAccount, amount: f64) {
//     println!(" new balance Add: {}", amount);
//     println!("loading.....");
//     account.balance += amount;
// }

// fn withdraw(account: &mut BankAccount, balance: f64) {
//     println!(" new balance after withdraw: {}", balance);
//     println!("loading.....");
//     account.balance -= balance
// }

// fn close_Account(account:&mut BankAccount,new_status:String ){
//  println!("close account loading .....");
//  account.status = new_status;

// }

// using impl

struct BankAccount {
    account_number: u32,
    owner_name: String,
    balance: f64,
    status: String,
}

impl BankAccount {
    fn create_new_account(account_number: u32, owner_name: String, balance: f64) -> BankAccount {
        BankAccount {
            account_number,
            owner_name,
            balance,
            status: String::from("Active"),
        }
    }
    fn view_balance(&self) {
        println!(" Owner Name: {}", self.owner_name);
        println!(" accountnumber: {}", self.account_number);
        println!(" balance: {}", self.balance);
        println!(" status: {}", self.status); // تم تعديل الكلمة هنا لـ status
    }
    fn deposit(&mut self, amount: f64) {
        println!(" new balance Add: {}", amount);
        println!("loading.....");
        self.balance += amount;
    }
    fn withdraw(&mut self, withdraw: f64) {
        println!(" new balance After withdraw: {}", withdraw);
        println!("loading.....");
        self.balance -= withdraw;
    }

    fn close_account(&mut self, new_status: String) {
        println!("close account loading .....");
        self.status = new_status;
    }
}
fn main() {
    let mut account=BankAccount::create_new_account(123, String::from("hazem hisham"),122.0);
    // old way for make new account
    // let mut account = BankAccount {
    //     account_number: 123,
    //     owner_name: String::from("hazem hisham"),
    //     balance: 500.0,
    //     status: String::from("Active"),
    // };
    println!("\n ---   account info ---");
    // بدلاً من: view_balance(&account);
    account.view_balance();

    println!("\n---   account add 200usd ---");
    // بدلاً من: deposit(&mut account, 200.0);
    account.deposit(200.0);

    println!("\n ---   new balance after deposit 200 ---");
    account.view_balance();

    println!("\n ---   new balance after withdraw 300 ---");
    account.withdraw(300.0);
    account.view_balance();

    println!("\n ---close account status ---");
    account.close_account(String::from("not active"));
    account.view_balance();
}
