struct Package {
    id: u32,
    destination: String,
    status: String,
}

fn main() {
    let mut package = Package {
        id: 120202,
        destination: String::from("Cairo"),
        status: String::from("Processing"),
    };

    println!("---   tracking info ---");

    print_package_status(&package);
    update_status(&mut package, String::from("shipped"));
    println!("\n--- بعد التحديث في الـ main ---");
    // طباعة الحالة مجدداً للتأكد من التحديث وأن الشحنة لم تمت
    print_package_status(&package);
}
fn print_package_status(package: &Package) {
    println!(" Package ID: {}", package.id);
    println!(" Destination: {}", package.destination);
    println!(" Status: {}", package.status);
}

fn update_status(package: &mut Package, new_status: String) {
    package.status = new_status;
    println!("\n[تم تحديث حالة الشحنة بنجاح في الـ Stack/Heap]");
}
