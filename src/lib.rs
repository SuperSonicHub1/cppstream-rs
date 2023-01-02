// Shaddup, Rust.
#![allow(unused_must_use)]
/// cout and cin in Rust.
/// See:
/// - https://en.cppreference.com/w/cpp/io/cout
/// - https://en.cppreference.com/w/cpp/io/cin

use std::ops::Shl;
use std::ops::Shr;
use std::ops::ShlAssign;
use std::io::stdout;
use std::io::Stdout;
use std::io::Write;
use std::io::stdin;
use std::io::Stdin;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
use std::fmt::Display;

pub struct COut {
    stdout: Stdout,
}

pub fn cout() -> COut {
    COut { stdout: stdout(), }
}

impl<T: Display> Shl<T> for COut {
    type Output = Self;
    
    fn shl(mut self, rhs: T) -> Self::Output {
        // I swear on my soul that this will never fail.
        self.stdout.write_fmt(format_args!("{}", rhs)).unwrap();
        self
    }
}


pub struct CIn {
    stdin: Stdin,
}

pub fn cin() -> CIn {
    CIn { stdin: stdin(), }
}

impl ShlAssign<CIn> for String {
    fn shl_assign(&mut self, CIn { stdin }: CIn) {
        // I swear on my soul that this will never fail.
        stdin.read_line(self).expect("Can't find a line.");
        // Get rid of newline
        self.pop();
    }
}

impl ShlAssign<CIn> for i32 {
    fn shl_assign(&mut self, CIn { stdin }: CIn) {
        let mut line = String::new();
        // I swear on my soul that this will never fail.
        // for stdin.

        stdin.read_line(&mut line).expect("Can't find a line.");
        *self = line.trim().parse().expect("Not a number.");
    }
}

impl Shr<&mut String> for CIn {
    type Output = Self;
    
    fn shr(self, rhs: &mut String) -> Self::Output {
        // I swear on my soul that this will never fail.
        self.stdin.read_line(rhs).expect("Can't find a line.");
        self
    }
}

impl Shr<&mut i32> for CIn {
    type Output = Self;
    
    fn shr(self, rhs: &mut i32) -> Self::Output {
        // I swear on my soul that this will never fail.

        let mut handle = self.stdin.lock();

        let mut chunk = Vec::new();
        while chunk.len() < 1 || String::from_utf8(chunk.to_vec()).expect("Invalid UTF-8.").trim().len() < 1 {
            handle.read_until(b' ', &mut chunk);
        }

        println!("{:?}", String::from_utf8(chunk.to_vec()).expect("Invalid UTF-8."));

        let converted = String::from_utf8(chunk).expect("Invalid UTF-8.");
        *rhs = converted.trim().parse().expect("Not a number.");            

        self
    }
}

#[cfg(windows)]
pub const endl: &'static str = "\r\n";
#[cfg(not(windows))]
pub const endl: &'static str = "\n";


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // cout() << "Hello world!" << endl << "GG ez m8." << endl;
        // cout() << "Hello world!" << endl() << "What's your name, boy?";
        let mut int1 = 0;
        let mut int2 = 0;
        cin() >> &mut int1 >> &mut int2;
        cout() << "\n";
        cout() << int1 << " " << int2 << endl;
        // cout() << "\n";
        // cout() << format!("You're {}, eh? Nice to meet ya!\nHow old are you? >", name);
        // let mut age = 0;
        // age <<= cin();
        // cout() << "\n";
        // cout() << format!("You're {}? You're my son's age; how sweet!\n", age);
    }
}
