use std::collections::HashMap;
use std::io;
struct Inventory {
    items: HashMap<String, u32>,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory {
            items: HashMap::new(),
        }
    }
    fn add_item(&mut self, name: &str, quantity: u32) {
        let count = self.items.entry(name.to_string()).or_insert(0);
        *count += quantity;
        println!("เติม {} เพิ่มอีก   {}   ชิ้น เรียบร้อย", name, quantity);
    }
    fn remove_item(&mut self, name: &str, quantity: u32) {
        if let Some(current_stock) = self.items.get_mut(name) {
            if *current_stock >= quantity {
                *current_stock -= quantity;
                println!(
                    "ขาย{} ออกไป{} ชิ้น    (คงเหลือ{})",
                    name, quantity, *current_stock
                );
            } else {
                println!("Eror :สินค้า{} มีไม่พอขาย(มีแค่ {})", name, *current_stock);
            }
        } else {
            println!("Eror : ไม่พบสินค้า '{}' ในคลัง", name);
        }
    }
    fn show_inventory(&self) {
        println!("\n --- รายการสินค้าคงคลัง---");
        if self.items.is_empty() {
            println!("คลังสินค้าว่างเปล่า...");
        }
        for (name, quantity) in &self.items {
            println!("- {}:{} ชิ้น", name, quantity);
        }
        println!("----------------------\n")
    }
    fn check_stock(&self, name: &str) -> Option<u32> {
        self.items.get(name).copied()
    }
}
fn main() {
    let mut my_store = Inventory::new();
    my_store.add_item("apple", 30);
    my_store.add_item("banana", 20);
    my_store.add_item("apple", 10);
    my_store.add_item("coffee", 50);
    my_store.show_inventory();

    my_store.remove_item("apple", 15);
    my_store.remove_item("banana", 5);
    my_store.show_inventory();

    let item_name = "coffee";
    match my_store.check_stock(item_name) {
        Some(quantity) => println!("สินค้า {} เหลืออยู่ {}   ชิ้น", item_name, quantity),
        None => println!("ขออภัยไม่พอนค้า  {} ", item_name),
    }
    if my_store.check_stock("coffee").is_none() {
        println!(" no more coffee")
    }
}
