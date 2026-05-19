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
enum OrderType {
    Market(f64), // يحمل الكمية فقط
    Limit { price: f64, amount: f64 }, // يحمل السعر والكمية كـ Struct
    StopLoss { trigger_price: f64, leverage: u32 }, // يحمل سعر التفعيل والرافعة المالية
}

fn main() {
    // 2. أنشئ أمر من نوع Limit هنا ببيانات من اختيارك
    let current_order = OrderType::Limit{price:145.5,amount:10.0};


    // 3. طابق الأمر واطبع البيانات
    match current_order {
        OrderType::Limit{price,amount} =>   println!("buying solana price {} \n qty {}", price, amount),
        OrderType::StopLoss{trigger_price,leverage}=>println!(" solana trigger  price {} \n with leverage  {}", trigger_price, leverage),
        OrderType::Market(quantity)=>println!(" solana   price {} \n ", quantity)
    }
}