#[derive(PartialEq, Debug)]
enum QueueStatus {
    Waiting,
    InProgress,
    Completed,
}
struct Customer {
    name: String,
    group_size: u32,
    status: QueueStatus
}
struct Restaurant {
    name: String,
    customers: Vec<Customer>
}
impl Restaurant{
    fn new(name:&str) -> Self {
        Restaurant {
            name: name.to_string(),
            customers: Vec::new()
        }
    }
    fn add_customer(&mut self, name: &str, group_size: u32) {
        let customer = Customer {
            name: name.to_string(),
            group_size,
            status: QueueStatus::Waiting
        };
        self.customers.push(customer);
        println!("Added customer: {} with group size: {}", name, group_size);   
    }
    fn call_next_customer(&mut self) {
        for customer in self.customers.iter_mut() {
            if customer.status == QueueStatus::Waiting {
                customer.status = QueueStatus::InProgress;
                println!("Calling next customer: {}", customer.name);
                return;
            }
        }
        println!("No customers waiting in the queue.");
    }
    fn show_status(&self) {
        println!("Restaurant: {}", self.name);
        for customer in &self.customers {
            let status = match customer.status {
                QueueStatus::Waiting => "Waiting",
                QueueStatus::InProgress => "In Progress",
                QueueStatus::Completed => "Completed"
            };
            println!("Customer: {}, Group Size: {}, Status: {}", customer.name, customer.group_size, status);
        }
    }  
}


fn main() {
    let mut restaurant = Restaurant::new("Gourmet Bistro");
    restaurant.add_customer("Alice", 4);
    restaurant.add_customer("Bob", 2);
    restaurant.add_customer("Charlie", 3);
    
    restaurant.show_status();
    
    restaurant.call_next_customer();
    restaurant.show_status();
}
