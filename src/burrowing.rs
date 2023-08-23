
// add a field to CustomerOrder type: i32 name: serial_id
struct CustomerOrder {
    order_status: String,
    quantity: i32,
    serial_id: i32,
}

impl CustomerOrder {
    fn new(order_status: &str, quantity: i32, serial_id: i32) -> Self { 
        Self { order_status: String::from(order_status), quantity: (quantity), serial_id: (serial_id) }
     } 
}

impl std::fmt::Display for CustomerOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CustomerOrder: order_status: {}, quantity: {}, serial_id: {}", self.order_status, self.quantity, self.serial_id)
    }
}

fn finish_order(order_to_complete: CustomerOrder) -> CustomerOrder {
    CustomerOrder {
        order_status: "complete".to_string(),
        ..order_to_complete
    }
}

fn burrowing() {
    let order = CustomerOrder::new("pending", 230, 50);
    println!("pending order: {}", order);

    // customer finished order ...

    let serial_id = finish_order(order);
    println!("finished order: {}", serial_id);
}