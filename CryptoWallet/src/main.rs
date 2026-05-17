struct CryptoWallet {
    coin_name: String,
    balance: f64,
}

fn main() {
    let mut my_wallet = CryptoWallet {
        coin_name: String::from("bitcoin"),
        balance: 0.5,
    };
    println!("---   system begin ---");
    buy_crypto(&mut my_wallet,1.4);
    print_wallet_report(&my_wallet);

    println!("\n--- after buying crypto    ---");
    print_wallet_report(&my_wallet);

}

fn print_wallet_report(wallet:&CryptoWallet){
   println!(" crypto currency: {}", wallet.coin_name);
    println!(" balance: {} BTC", wallet.balance);
}

fn buy_crypto (wallet:&mut CryptoWallet,amount:f64){
   println!("\n[buying crypto currency in wallet: {} {}.....]", amount,wallet.coin_name);
   wallet.balance+=amount;

}