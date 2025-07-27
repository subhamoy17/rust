use std::collections::HashMap;

// -------- Logger Trait & Implementations -------- //
trait Logger {
    fn log(&self, message: &str);
}

struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("{}", message);
    }
}

// -------- Error Handling -------- //
#[derive(Debug)]
enum WalletError {
    IncorrectPassword,
    InsufficientFunds,
    UserNotFound,
    SameSenderReceiver,
}

// -------- Transaction Struct -------- //
#[derive(Debug, Clone)]
struct Transaction {
    to: String,
    amount: f64,
}

// -------- User Struct -------- //
struct User {
    username: String,
    password: String,
    balance: f64,
    history: Vec<Transaction>,
}

impl User {
    fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            balance: 0.0,
            history: Vec::new(),
        }
    }

    fn authenticate(&self, password: &str) -> Result<(), WalletError> {
        if self.password == password {
            Ok(())
        } else {
            Err(WalletError::IncorrectPassword)
        }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn show_history(&self) {
        for txn in &self.history {
            println!("Sent {} to {}", txn.amount, txn.to);
        }
    }
}

// -------- Wallet Application -------- //
struct WalletApp {
    users: HashMap<String, User>,
    logger: Box<dyn Logger>,
}

impl WalletApp {
    fn new(logger: Box<dyn Logger>) -> Self {
        Self {
            users: HashMap::new(),
            logger,
        }
    }

    fn create_user(&mut self, username: &str, password: &str) {
        self.users.insert(
            username.to_string(),
            User::new(username, password),
        );
        self.logger.log(&format!("User '{}' created.", username));
    }

    fn deposit(&mut self, username: &str, amount: f64) {
        if let Some(user) = self.users.get_mut(username) {
            user.deposit(amount);
            self.logger.log(&format!("{} deposited {}.", username, amount));
        }
    }

    fn check_balance(&self, username: &str, password: &str) -> Result<f64, WalletError> {
        let user = self.users.get(username).ok_or(WalletError::UserNotFound)?;
        user.authenticate(password)?;
        Ok(user.balance)
    }

    fn transfer(
        &mut self,
        from: &str,
        to: &str,
        amount: f64,
        password: &str,
    ) -> Result<(), WalletError> {
        if from == to {
            return Err(WalletError::SameSenderReceiver);
        }

        // Temporarily remove sender to avoid double mutable borrow
        let mut sender = self.users.remove(from).ok_or(WalletError::UserNotFound)?;
        sender.authenticate(password)?;

        if sender.balance < amount {
            // Put sender back before returning error
            self.users.insert(from.to_string(), sender);
            return Err(WalletError::InsufficientFunds);
        }

        let receiver = self.users.get_mut(to).ok_or(WalletError::UserNotFound)?;

        // Perform transfer
        sender.balance -= amount;
        receiver.balance += amount;

        sender.history.push(Transaction {
            to: to.to_string(),
            amount,
        });

        // Reinsert sender after mutation
        self.users.insert(from.to_string(), sender);

        self.logger.log(&format!("{} transferred {} to {}", from, amount, to));
        Ok(())
    }

    fn show_history(&self, username: &str, password: &str) -> Result<(), WalletError> {
        let user = self.users.get(username).ok_or(WalletError::UserNotFound)?;
        user.authenticate(password)?;
        user.show_history();
        Ok(())
    }
}

// -------- Main Simulation -------- //
fn main() {
    let mut app = WalletApp::new(Box::new(ConsoleLogger));

    app.create_user("alice", "alice123");
    app.create_user("bob", "bob123");

    app.deposit("alice", 1000.0);

    match app.check_balance("alice", "alice123") {
        Ok(bal) => println!("Alice's Balance: {:.2}", bal),
        Err(e) => println!("Error: {:?}", e),
    }

    match app.transfer("alice", "bob", 300.0, "alice123") {
        Ok(_) => println!("Transfer successful!"),
        Err(e) => println!("Transfer failed: {:?}", e),
    }

    match app.check_balance("bob", "bob123") {
        Ok(bal) => println!("Bob's Balance: {:.2}", bal),
        Err(e) => println!("Error: {:?}", e),
    }

    println!("\nTransaction History for Alice:");
    let _ = app.show_history("alice", "alice123");
}
