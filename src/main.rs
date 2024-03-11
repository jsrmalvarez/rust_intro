use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
struct User<'a> {
	name: &'a String, // Struct with a must specify lifetimes
}

fn main() {
	let user_1_name = String::from("Meg");
	let user_2_name = String::from("Eve(Admin)");
	let user_3_name = user_1_name.clone();

    let user_1 = User { name: &user_1_name }; // Cannot borrow after transferring ownership
    let user_2 = User { name: &user_2_name };
	let user_3 = User { name: &user_3_name };


	let mut database = HashMap::<&String, &[u32]>::default() ;  // Initialization required
	database.insert(user_1.name, &[1,2,3,4,5]); // Immutable by default
	database.insert(user_2.name, &[42,43,44,45,46,47,48]);

	// Option<T> can be determined to be Some(T) or None
	match database.get(user_1.name) {
		Some(user_data) => {
				println!("Printing {}'s data", user_1.name);
				print_user_data_safe_implementation(user_data, 20)
		}, // Containers are bound-checked
		None => panic!("User not found"), // Patterns must be exhaustively covered!
	}

	println!("{}", user_3.name);


	// Result<T, Err> can be verified to be Ok(T) or Err
	match database.try_reserve(500) {
		Ok(()) => (),
		Err(_) => panic!("Cannot reserve additional database records"),
	}

	// Dirty things must be inside an unsafe block
	let x = 42;
	unsafe {
		// Example: dereferencing a raw pointer
		let raw_ptr = &x as *const i32;
		println!("Unsafe dereference: {}", *(raw_ptr.add(1)));
	}

}

fn print_user_data_safe_implementation(user_data:&[u32], len:usize){

    for n in 0..len {
        println!("{:?}", user_data[n]);
    }
}

fn print_user_data_unsafe_implementation(user_data:&[u32], len:usize){

    for n in 0..len {
		unsafe{
			let raw_ptr = (&user_data[0] as *const u32).add(n);
			println!("{:?}", *raw_ptr);
		}
    }
}