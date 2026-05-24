use std::fmt; // استدعاء مكتبة التنسيق والطباعة

// 1. الـ Enum بتاعنا
enum NetworkState {
    Connected,
    Connecting,
    Disconnected,
}

// 2. بتلقن الموظف (Display) الكتالوج اللي هيمشي عليه
impl fmt::Display for NetworkState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetworkState::Connected => write!(f, "🟢 Network is Online and Stable"),
            NetworkState::Connecting => write!(f, "🟡 Attempting to reconnect to Binance servers..."),
            NetworkState::Disconnected => write!(f, "🔴 CRITICAL: Network is Offline! Trading paused."),
        }
    }
}

// 3. الـ Struct اللي بيشيل بيانات الإشارة
struct BotSignal {
    bot_id: u32,
    network: NetworkState,
}

impl BotSignal {
    // ميثود بتطبع الـ Log للمستخدم
    fn emit_log(&self) {
        // راقب هنا: استخدمنا {} العادية مع حالة الشبكة لأننا علمنا رست الـ Display خلاص!
        println!("🤖 [Bot #{}] Status Report -> {}", self.bot_id, self.network);
    }
}

// 4. الـ main الرايقة والنظيفة:
fn main() {
    // هنعمل بوت في حالة اتصال
    let bot_1 = BotSignal {
        bot_id: 101,
        network: NetworkState::Connecting,
    };

    // هنعمل بوت تاني متصل ومستقر
    let bot_2 = BotSignal {
        bot_id: 102,
        network: NetworkState::Connected,
    };

    // اطبع الـ Logs مباشرة وشوف السحر
    bot_1.emit_log();
    bot_2.emit_log();
}