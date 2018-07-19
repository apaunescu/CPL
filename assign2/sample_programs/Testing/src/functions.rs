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

fn merge(fst : Vec<u32>, snd : Vec<u32>) -> Vec<u32>{
	let mut list = vec![];
	let mut i = 0;
	let mut j = 0;
	
	if snd == []{
		return fst
	}
	if fst == []{
		return snd
	}

	while fst.get(i) != None && snd.get(j) != None{
		if fst[i] >= snd[j]{
			list.push(snd[j]);
			j += 1;
		}
		else{
			list.push(fst[i]);
			i += 1;
		}
	}

	for a in i..fst.len(){
		list.push(fst[a]);
	}

	for b in j..snd.len(){
		list.push(snd[b]);
	}

	return list
}

fn mergesort(list : Vec<u32>) -> Vec<u32>{
	let mut sorted = vec![];

	if list == []{
		return list
	}
	if list.len() == 1{
		return list
	}
	let (fst, snd) = list.split_at(list.len() / 2);

	sorted = merge(mergesort(fst.to_vec()), mergesort(snd.to_vec()));

	return sorted
}


