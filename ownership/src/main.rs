// // stack and heap
// fn main() {
//     // stack have specifi size and is faster than heap
//     let age: i32 = 30;
//     let is_married: bool = false;
    

//     //    heap have dynamic size and is slower than stack
//     let mut name:String=String::from("John Doe");
//     let mut gender: String = String::from("male");
//     let mut dynamic_list: Vec<i32> = vec![1, 2, 3, 4, 5];

//     // we can change the value of gender    
//     name.push_str(" and female");
//     // we can also add more element to dynamic_list
//     dynamic_list.push(6);

//     println!("Age: {}", age);
//     println!("Is married: {}", is_married);
//     println!("Name: {}", name);
//     println!("Gender: {}", gender);
//     println!("Dynamic list: {:?}", dynamic_list);
// }

// fn main(){
//     // name is only valid within this block
//     {
//         let name:String=String ::from("John Doe");
//         println!("Name inside block: {}", name);
//     }
//     // name is not valid here, it has been dropped
//     //  println!("Name outside block: {}", name); // This will cause a compile error

// }

// fn main(){
//     let s =take_admin();
//     let s2=String::from("admin");;
//     let s3 = takes_and_gives_back(&s2);
//     println!("Admin: {}", s);
//     println!("Admin2: {}", s2);
//     println!("Admin3: {}", s3);
// }

// fn take_admin() -> String {
//     let admin = String::from("admin");
//     admin
// }
// fn takes_and_gives_back(admin: &String) -> usize {
//    admin.len()
// }

// struct BankAccount {
//     owner: String,
//     balance: f64,
// }

// fn main(){
//     let account =BankAccount {
//         owner: String::from("Alice"),
//         balance: 1000.0,
//     };
//     println!("Account owner: {}", account.owner);
//     println!("Account balance: {}", account.balance);
//     print_statement(&account);
//     println!("Account balance: {}", account.balance); // لا يزال بإمكاننا الوصول إلى الحساب بعد استدعاء الدالة، لأننا لم ننقل ملكيته، بل قمنا بتمريره كمستعير (reference)
// }

// fn print_statement(acc: &BankAccount) {
//     println!("--- تقرير الحساب البنكي ---");
//     println!("العميل: {}", acc.owner);
//     println!("الرصيد الحالي: ${}", acc.balance);
//     println!("---------------------------");
// } // ينتهي نطاق الدالة هنا، ويموت المستعير `acc`، لكن الحساب الأصلي يظل آمناً في الـ main

fn main(){
    let mut  name:String=String::from("hazem");
    print_name(&mut name)
}

fn print_name(name:&mut String){
    name.push_str("wlecome");
    print!("{}",name)
}