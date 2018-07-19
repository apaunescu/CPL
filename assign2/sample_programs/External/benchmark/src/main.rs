extern crate time;
extern crate libc;


use time::{Duration, PreciseTime};

//We import our C function and make it input and output c type ints.

extern {
	fn fibonacci(input: libc::c_int) -> libc::c_int;
}

fn fib(value : i32) -> i32 {
	if value == 0 {
		return 0
	}
	let mut f1 = 0;
	let mut f2 = 1;
	let mut f3 = f1 + f2;
	for i in 1..value {
		f3 = f1 + f2;
		f1 = f2;
		f2 = f3;
	}
	return f3
}

//Actually Running our function below.

fn main() {
    let input = 11;
    let input2 = 13;
    let input3 = 4;	
    let input4 = 0;

    unsafe {
    	//We test and see our C code works
    	println!("fibonacci of {:?} is {:?}", input, fibonacci(input));
    	println!("fibonacci of {:?} is {:?}", input2, fibonacci(input2));
    	println!("fibonacci of {:?} is {:?}", input3, fibonacci(input3));
    	println!("fibonacci of {:?} is {:?}", input4, fibonacci(input4));

    	//We time how long it takes to run this in C
    	let start = PreciseTime::now();
		for i in 0..1000 {
			fibonacci(input);
			fibonacci(input2);
			fibonacci(input3);
			fibonacci(input4);
		}
		let end = PreciseTime::now();
		println!("{} to run 4 (smallish) fibonacci sequences 1000 times in C code.", start.to(end));
    }

	//We take the time again, this time using Rust Code
	let start = PreciseTime::now();
	for i in 0..1000 {
		fib(input);
		fib(input2);
		fib(input3);
		fib(input4);
	}
	let end = PreciseTime::now();
	println!("{} to run 4 (smallish) fibonacci sequences 1000 times in Rust code.", start.to(end));

}


