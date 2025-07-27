use std::io;
use std::error::Error;
// Custom Error Enum
#[derive(Debug)]
enum LoanError {
    AgeError,
    IncomeError,
}
use std::fmt;

impl fmt::Display for LoanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoanError::AgeError => write!(f, "Age must be between 21 and 60."),
            LoanError::IncomeError => write!(f, "Income must be at least ₹15,000."),
        }
    }
}
impl Error for LoanError {}
struct Applicant {
    name: String,
    age: u32,
    income: u32,
    co_applicant: Option<Box<Applicant>>,
}

impl Applicant {
    fn is_eligible(&self) -> Result<(), LoanError> {
        if self.age < 21 || self.age > 60 {
            return Err(LoanError::AgeError);
        }
        if self.income < 15000 {
            return Err(LoanError::IncomeError);
        }
        Ok(())
    }
}

fn read_input(prompt: &str) -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn read_input_u32(prompt: &str) -> Result<u32, Box<dyn Error>> {
    let input = read_input(prompt)?;
    let number: u32 = input.parse()?;
    Ok(number)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Loan Approval System ---");
    let name = read_input("Enter your name:")?;
    let age = read_input_u32("Enter your age:")?;
    let income = read_input_u32("Enter your monthly income (₹):")?;
    let loan_amount = read_input_u32("Enter desired loan amount (₹):")?;
    let has_coapp = read_input("Do you have a co-applicant? (yes/no):")?;
    let mut co_applicant: Option<Box<Applicant>> = None;
    if has_coapp.to_lowercase() == "yes" {
        let co_name = read_input("Enter co-applicant name:")?;
        let co_age = read_input_u32("Enter co-applicant age:")?;
        let co_income = read_input_u32("Enter co-applicant income (₹):")?;

        co_applicant = Some(Box::new(Applicant {
            name: co_name,
            age: co_age,
            income: co_income,
            co_applicant: None,
        }));
    }
    let applicant = Applicant {
        name,
        age,
        income,
        co_applicant,
    };

    match applicant.is_eligible() {
        Ok(_) => println!("Yes, {} is eligible for loan of ₹{}", applicant.name, loan_amount),
        Err(e) => match &applicant.co_applicant {
            Some(co) => {
                while let Err(err) = co.is_eligible() {
                    match err {
                        LoanError::AgeError => {
                            println!("Both applicants failed: Age constraint not met.");
                            return Err(Box::new(err));
                        }
                        LoanError::IncomeError => {
                            println!("Both applicants failed: Income too low.");
                            return Err(Box::new(err));
                        }
                    }
                }
                println!("✅ {} (Co-applicant) is eligible. Loan of ₹{} approved.", co.name, loan_amount);
            }
            None => match e {
                LoanError::AgeError => println!("Loan rejected: Age must be between 21 and 60."),
                LoanError::IncomeError => println!("Loan rejected: Income must be at least ₹15,000."),
            },
        },
    }

    Ok(())
}
