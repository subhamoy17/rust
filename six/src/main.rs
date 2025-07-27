use std::fmt;
use std::panic::{self, AssertUnwindSafe};

//  Seat Definition
#[derive(Debug, Clone)]
struct Seat {
    number: usize,
    passenger_name: String,
}

// Custom Errors 
#[derive(Debug)]
enum SeatError {
    AlreadyBooked,
    InvalidSeatNumber,
    SeatNotBooked,
}

impl fmt::Display for SeatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SeatError::AlreadyBooked => write!(f, "Seat already booked"),
            SeatError::InvalidSeatNumber => write!(f, "Invalid seat number"),
            SeatError::SeatNotBooked => write!(f, "Seat not yet booked"),
        }
    }
}

// Plane Struct
struct Plane {
    seats: Vec<Option<Seat>>,
}

impl Plane {
    fn new(total_seats: usize) -> Self {
        Plane {
            seats: vec![None; total_seats],
        }
    }

    fn book_seat(&mut self, seat_number: usize, passenger_name: String) -> Result<(), SeatError> {
        if seat_number >= self.seats.len() {
            return Err(SeatError::InvalidSeatNumber);
        }

        if let Some(_) = &self.seats[seat_number] {
            return Err(SeatError::AlreadyBooked);
        }

        self.seats[seat_number] = Some(Seat {
            number: seat_number,
            passenger_name,
        });

        println!(" Seat {} successfully booked.", seat_number);
        Ok(())
    }

    fn cancel_seat(&mut self, seat_number: usize) -> Result<(), SeatError> {
        if seat_number >= self.seats.len() {
            return Err(SeatError::InvalidSeatNumber);
        }

        if let Some(_) = &self.seats[seat_number] {
            self.seats[seat_number] = None;
            println!(" Seat {} cancelled.", seat_number);
            Ok(())
        } else {
            Err(SeatError::SeatNotBooked)
        }
    }

    fn display_seats(&self) {
        println!("\n Seat Map:");
        for (index, seat) in self.seats.iter().enumerate() {
            match seat {
                Some(seat) => println!(" Seat {:02}: Booked by {}", index, seat.passenger_name),
                None => println!(" Seat {:02}: Available", index),
            }
        }
    }
}

// Panic-safe Overbooking Simulation
fn attempt_overbook_safe(plane: &mut Plane) {
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        for i in 0..=plane.seats.len() {
            let name = format!("Passenger{}", i);
            match plane.book_seat(i, name) {
                Ok(_) => {}
                Err(e) => {
                    panic!("Overbooking error at seat {}: {}", i, e);
                }
            }
        }
    }));

    match result {
        Ok(_) => println!(" Booking finished safely."),
        Err(_) => println!(" Panic caught: Attempted overbooking!"),
    }
}

fn main() {
    let mut plane = Plane::new(5);

    plane.display_seats();

    plane.book_seat(0, "Alice".to_string()).unwrap();
    plane.book_seat(1, "Bob".to_string()).unwrap();
    plane.book_seat(2, "Charlie".to_string()).unwrap();

    plane.display_seats();

    match plane.cancel_seat(1) {
        Ok(_) => println!(" Seat 1 canceled."),
        Err(e) => println!(" Error: {}", e),
    }

    plane.display_seats();
    // Simulate overbooking and catch panic safely
    println!("\n Simulating overbooking:");
    attempt_overbook_safe(&mut plane);
}

