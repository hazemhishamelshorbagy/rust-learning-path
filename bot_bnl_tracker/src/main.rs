
#[derive(Debug)]
enum BotStatus {
    Active,
    Paused,
}

struct BotTracker {
    bot_name: String,
    status: BotStatus,
    current_balance: f64,
}

impl BotTracker {
    fn create_tracker(bot_name: String, status: BotStatus, current_balance: f64) -> BotTracker {
        BotTracker {
            bot_name,
            status,

            current_balance,
        }
    }

    fn view_tracker_coin(&self) {
        println!(" Bot Name: {}", self.bot_name);
        println!(" status: {:?}", self.status);
        println!(" current_balance: ${:.2}", self.current_balance);
    }

    fn calculate_pnl(&mut self, initial_capital: f64) {
        println!(" calculating: {}", initial_capital);
        println!("loading.....");
        let pnl = self.current_balance - initial_capital;
        match self.status {
            BotStatus::Active => {
                println!("🟢 Bot is Still working");
                println!(" -> BOT Name: {}", self.bot_name);
                println!("🟢 Total PnL: ${:.2}", pnl);
            }
            BotStatus::Paused => {
                println!("🔴 Bot is Stopped");
                println!(" -> Stop on Balance: ${:.2}", self.current_balance);
                println!("🔴 Final PnL was: ${:.2}", pnl);
            }
        }
    }
}

fn main() {
    let mut my_bot =
        BotTracker::create_tracker("SOL_Grid_01".to_string(), BotStatus::Paused, 225.55);

    // عرض بيانات البوت
    my_bot.view_tracker_coin();
    println!("--------------------------------------");

    // حساب الأرباح والخسائر بناءً على رأس مال 210$
    my_bot.calculate_pnl(210.0);
}
