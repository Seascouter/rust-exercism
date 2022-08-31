pub fn create_empty() -> Vec<u8> {
    return vec!();
}

pub fn create_buffer(count: usize) -> Vec<u8> {
    let buffer = vec![0; count];
    return buffer;
}

pub fn create_fibonacci() -> Vec<u8> {
    let mut buffer = create_buffer(5);
    for i in 0..5 {
	if i < 2 {
	    buffer[i] = 1;
	} else {
	    buffer[i] = buffer[i-1] + buffer[i-2];
	}
    }
    return buffer;
}


fn main() {
    let fibonacci = create_fibonacci();
    for i in &fibonacci {
	println!("{}", i);
    }
}
