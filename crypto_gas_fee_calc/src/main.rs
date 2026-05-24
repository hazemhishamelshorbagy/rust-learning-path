
use std::fmt;
enum GasSpeed{
    Slow,
    Standard,
    Fast
}

impl fmt::Display for GasSpeed{
  fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
    match self{
       GasSpeed::Slow => write!(f, "🐢 Low Priority (Takes up to 10 mins)"), 
       GasSpeed::Standard => write!(f,"⚡ Medium Priority (Takes up to 2 mins)"),
       GasSpeed::Fast => write!(f,"🚀 Instant Execution (Takes seconds)")
    }
  }
}

struct Transaction{
    tx_id:u64,
    amount:f64,
    speed:GasSpeed
}

impl Transaction{
    fn print_receipt(&self){
        println!(
            "📜 Tx ID: #{}\n💰 Amount: {} SOL\n⛽ Gas Speed: {}", 
            self.tx_id, self.amount, self.speed
        );
    }
}

fn main() {
   let bot_1=Transaction{
   tx_id:123,amount:12.20,speed:GasSpeed::Fast
   };

    bot_1.print_receipt();
}
