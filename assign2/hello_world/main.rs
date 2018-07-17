fn main() {
	println!("divisors 30: {:?}", divisors(30));
	println!("divisors 64: {:?}", divisors(64));
	println!("divisors 127: {:?}", divisors(127));

	println!("primes 7: {:?}", primes(7));
	println!("primes 100: {:?}", primes(100));

	println!("join , [one,two,three]: {}", join("," ,["one","two","three"].to_vec()));
	println!("join + [1,2,3]: {}", join("+" ,["1","2","3"].to_vec()));
	println!("join X [abc]: {}", join("X" ,["abc"].to_vec()));
	println!("join X []: {}", join("X" ,[].to_vec()));

	println!("pythagorean 10: {:?}", pythagorean(10));
	println!("pythagorean 30: {:?}", pythagorean(30));

	println!("hailSeq 1: {:?}", hailseq(1));
	println!("hailSeq 11: {:?}", hailseq(11));
	println!("hailSeq 6: {:?}", hailseq(6));

	println!("fib 0: {:?}", fib(0));
	println!("fib 1: {:?}", fib(1));
	println!("fib 2: {:?}", fib(2));
	println!("fib 3: {:?}", fib(3));
	println!("fib 10: {:?}", fib(10));
	println!("fib 20: {:?}", fib(20));


}

fn divisors(value : u32) -> Vec<u32> {
	let mut list = vec![];
	for i in 2..value {
		if value % i == 0 {
			list.push(i);
		}
	}
	return list
}

fn primes(value : u32) -> Vec<u32> {
	let mut list = vec![];
	for i in 2..value+1 {
		if divisors(i) == [] {
			list.push(i);
		}
	}
	return list
}

fn join(seperator : &str, list : Vec<&str>) -> String {
	let joined = list.connect(seperator);
	return joined
}

fn pythagorean(value : u32) -> Vec<(u32, u32, u32)> {
	let mut list = vec![];
	for a in 1..value+1 {
		for b in 1..value+1{
			for c in 1..value+1{
				if a.pow(2) + b.pow(2) == c.pow(2) && a <= b && b <= c{
					list.push((a,b,c))
				}
			}
		}
	}
	return list
}

fn hailstone(value : u32) -> u32{
	if value % 2 == 0 {
		value / 2
	}
	else{
		value * 3 + 1
	}
}

fn hailseq(value : u32) -> Vec<u32> {
	let mut list = vec![];
	let mut new_val = value;
	list.push(new_val);
	if value == 1 {
		return list
	}
	while hailstone(new_val) != 1 {
		list.push(hailstone(new_val));
		new_val = hailstone(new_val);
	}
	list.push(1);
	return list	
}

fn fib(value : u32) -> u32 {
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
