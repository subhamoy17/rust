use std::boxed::Box;
struct Cash {
    total: u32,
}
impl Cash {
    fn new(amount: u32) -> Self {
        println!("Cash created with ₹{}", amount);
        Self { total: amount }
    }

    fn withdraw(&mut self, amount: u32) -> Result<u32, String> {
        println!("Cash withdraw Cash object at: {:p} (on heap if boxed, otherwise on stack)",self);
        println!("Cash withdraw total field address: {:p}",&self.total);

        if amount <= self.total {
            self.total -= amount;
            println!("Withdrawn: ₹{}, Remaining: ₹{}", amount, self.total);
            Ok(amount)
        } else {
            Err(format!("Insufficient balance. Available: ₹{}", self.total))
        }
    }
}
impl Drop for Cash {
    fn drop(&mut self) {
        println!("Cash drop Releasing Cash at {:p}. ₹{} returned to vault.", self, self.total);
    }
}
struct ATM {
    cash: Box<Cash>,
}
impl ATM {
    fn new(initial_cash: u32) -> Self {
        let cash = Box::new(Cash::new(initial_cash));
        println!("ATM new Heap address: {:p}, stack address: {:p}",&*cash, &cash);
        Self { cash }
    }
    fn process_withdrawal(&mut self, amount: u32) {
        println!("ATM process_withdrawal ATM instance at: {:p}",self);
        match self.cash.withdraw(amount) {
            Ok(_) => println!("Withdrawal successful."),
            Err(e) => println!("Error: {}", e),
        }
    }
}
fn main() {
    let mut atm = ATM::new(5000); // ATM initial amount
    atm.process_withdrawal(1200);
    atm.process_withdrawal(3000);
    atm.process_withdrawal(1000);
}
