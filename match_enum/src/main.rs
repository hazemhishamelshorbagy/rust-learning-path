// enum TrafficLight{
//     Red,
//     Yellow,
//     Green,
// }

// fn main(){
//     let light = TrafficLight::Red;
//     match light{
//         TrafficLight::Red => println!("stop it is red light "),
//         TrafficLight::Yellow => println!("be ready it is yellow light "),
//         TrafficLight::Green => println!("go it is green light  "),
//     }

// }

// enum Drink {
//     Espresso,
//     Latte(String),       // يحمل نوع الحليب كنص
//     Cappuccino(u32),
// }

// fn main(){
//     let client_order = Drink::Cappuccino(3);
//     match client_order{
//       Drink::Espresso =>   println!("your drink espresso ready "),
//       Drink::Latte(milk_type) =>  println!("your drink Latte ready with {}",milk_type),
//       Drink ::Cappuccino(sugar_count) =>  println!("your drink Cappuccino ready with  {} sugar",sugar_count),
//     }
// }

// enum PowerUp{
//     Normal,
//     Shield(u32),
//     SpeedMultiplier(f64)
// }

// fn main(){
//   let player = PowerUp::Shield(50);
//   match player{
//     PowerUp::Shield(shield_pow) =>  println!("your shield power be {} ",shield_pow),
//     PowerUp::Normal=> println!("your  power is Normal"),
//     PowerUp::SpeedMultiplier(speed_pow) =>println!("your  speed increased by   {} ",speed_pow)
//   }
// }

// enum ShippingStatus{
//     Processing,
//     InTransit(u32),
//     Delivered
// }

// fn main(){
//     let track_info=ShippingStatus::InTransit(3);

//     match track_info{
//         ShippingStatus::InTransit(no_of_days) => println!("your  track will arivved  in {} days ",no_of_days),
//         ShippingStatus::Processing =>println!("your  in processing...."),
//         ShippingStatus::Delivered =>println!("Deilivered"),
//     }
// }

// enum LoginRole{
//     Guest,
//     User(String),
//     Admin { name: String, level: u32 }
// }

// fn main(){
//     let user_role=LoginRole::Admin{name:String::from("Ahmed"),level:5};
//     match user_role{
//        LoginRole::Guest => println!("Welcome our guest "),
//        LoginRole::User(name) => println!("Welcome our user {} ",name),
//       LoginRole::Admin { name, level } => {
//             println!("Welcome our admin {}\nyour lvl {}", name, level);
//         }
//     }
// }

// Trading order system
// 1. تعريف الـ Enum الخاص بأوامر التداول
// enum OrderType {
//     Market(f64), // يحمل الكمية فقط
//     Limit { price: f64, amount: f64 }, // يحمل السعر والكمية كـ Struct
//     StopLoss { trigger_price: f64, leverage: u32 }, // يحمل سعر التفعيل والرافعة المالية
// }

// fn main() {
//     // 2. أنشئ أمر من نوع Limit هنا ببيانات من اختيارك
//     let current_order = OrderType::Limit{price:145.5,amount:10.0};

//     // 3. طابق الأمر واطبع البيانات
//     match current_order {
//         OrderType::Limit{price,amount} =>   println!("buying solana price {} \n qty {}", price, amount),
//         OrderType::StopLoss{trigger_price,leverage}=>println!(" solana trigger  price {} \n with leverage  {}", trigger_price, leverage),
//         OrderType::Market(quantity)=>println!(" solana   price {} \n ", quantity)
//     }
// }

// enum OrderType {
//     Market(f64),
//     Limit { price: f64, amount: f64 },
//     StopLoss { trigger_price: f64, leverage: u32 },
// }

// impl OrderType {
//     fn calculate_total(&self) -> f64 {
//         match self {
//             OrderType::Market(amount) => {
//                 let current_market_price = 145.0;
//                 amount * current_market_price
//             }
//             OrderType::Limit { price, amount } => {
//                 price * amount
//             }
//             OrderType::StopLoss { trigger_price, .. } => {
//                 *trigger_price
//             }
//         }
//     }
// }

// fn main() {
//     let limit_order = OrderType::Limit { price: 150.0, amount: 2.0 };
//     let market_order = OrderType::Market(5.0);

//     println!("Limit Order Total: ${}", limit_order.calculate_total());
//     println!("Market Order Total: ${}", market_order.calculate_total());
// }

// Fin wallet
// struct Wallet {
//     public_address: String,
//     bitcoin_balance: f64,
// }

// fn find_wallet(address: &str) -> Option<Wallet> {
//     if address == "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa" {
//         Some(Wallet {
//             public_address: String::from(address),
//             bitcoin_balance: 1.15,
//         })
//     } else {
//         None // لم نجد شيئاً!
//     }
// }

// fn main() {
//     let search_result = find_wallet("unknown_address");
//     match search_result {
//         Option::Some(wallet) => {
//             println!("Wallet found! Balance: {} BTC", wallet.bitcoin_balance);
//         }
//         Option::None => {
//             println!("Error: Wallet address not found in our database!");
//         }
//     }
// }

// دالة حساب نسبة التغير بأمان
fn calculate_pnl(entry_price: f64, current_price: f64) -> Option<f64> {
    if entry_price <= 0.0 {
        None // القسمة على صفر أو سعر سالب غير ممكن، نعيد None بأمان
    } else {
        let change_percentage = ((current_price - entry_price) / entry_price) * 100.0;
        Some(change_percentage) // الحسبة سليمة، نغلف الناتج داخل Some
    }
}

fn main() {
    let current_sol_price = 150.0;
    
    // السيناريو الأول: سعر دخول صحيح (120$)
    let trade_1 = calculate_pnl(120.0, current_sol_price);
    
    // السيناريو الثاني: خطأ في البيانات، سعر الدخول صفر!
    let trade_2 = calculate_pnl(0.0, current_sol_price);

    // فحص الصفقة الأولى
    match trade_1 {
        Some(pnl) => println!("Trade 1 PnL: {:.2}%", pnl), // ستطبع: 25.00%
        None => println!("Trade 1 Error: Invalid entry price!"),
    }

    // فحص الصفقة الثانية
    match trade_2 {
        Some(pnl) => println!("Trade 2 PnL: {:.2}%", pnl),
        None => println!("Trade 2 Error: Cannot calculate PnL, entry price is zero!"), // ستمسك الخطأ هنا بسلام
    }
}