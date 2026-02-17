enum ProductCategory {
    Food,
    Electronics,
    Clothing,
}

struct Product {
    name: String,
    price: f64,
    category: ProductCategory,
}
impl Product {
    fn new(name: &str, price: f64, category: ProductCategory) -> Product {
        Product {
            name: name.to_string(),
            price,
            category,
        }
    }

    // ฟังก์ชันคำนวณราคาหลังรวมภาษี (Food ภาษี 0%, อื่นๆ 7%)
    fn price_with_tax(&self) -> f64 {
        match self.category {
            ProductCategory::Food => self.price,
            _ => self.price * 1.07,
        }
    }
}

// 4. ฟังก์ชันค้นหาสินค้า (จำลองว่าคืนค่าเป็น Option)
fn find_product(id: i32) -> Option<Product> {
    if id == 1 {
        Some(Product::new("Apple", 20.0, ProductCategory::Food))
    } else if id == 2 {
        Some(Product::new(
            "Laptop",
            30000.0,
            ProductCategory::Electronics,
        ))
    } else {
        None
    }
}

fn main() {
    let search_id = 3;
    let result = find_product(search_id);

    match result {
        Some(p) => {
            println!(" --- found product ---");
            println!("name : {}", p.name);
            println!("price include tax : {:.2} baht", p.price_with_tax());
        }
        None => println!("--- not found product ---"),
    }
}
