enum TradeType {
    Buy,
    Sell,
}
struct Trade {
    id: u32,
    pair: String,
    price: f64,
    amount: f64,
    trade_type: TradeType,
}

impl Trade {
    fn generate(id: u32, pair: String, price: f64, amount: f64, trade_type: TradeType) -> Trade {
        Trade {
            id,
            pair,
            price,
            amount,
            trade_type,
        }
    }
    fn calculate_value(&self) {
        let total_cal = self.amount * self.price;
        match self.trade_type {
            TradeType::Buy => {
                println!(
                    "🟢 BOT ACTION: Buying {} worth ${:.2}",
                    self.pair, total_cal
                );
                println!("   -> Adding positions to your wallet.")
            },
            TradeType::Sell=>{
                println!("🔴 BOT ACTION: Selling {} worth ${:.2}", self.pair, total_cal);
                println!("   -> Taking profits/cutting losses.")
            }
        }
    }
}
fn main() {
   let buy_trade = Trade::generate(1, "SOL/USDT".to_string(), 180.0, 1.5, TradeType::Buy);
    buy_trade.calculate_value();

    println!("--------------------------------------------------");

    // 2. هنعمل صفقة بيع (Sell)
    let sell_trade = Trade::generate(2, "SOL/USDT".to_string(), 185.5, 1.5, TradeType::Sell);
    sell_trade.calculate_value();
}
