use std::collections::HashMap;

// To be used as a key in a HashMap, must implement Hash, PartialEq, Eq
struct User {
	name: &String, // Struct with a must specify lifetimes
}

fn main() {
	let user_1_name = String::from("Meg");
	let user_2_name = String::from("Eve(Admin)");
	let user_3_name = user_1_name;

    let user_1 = User { name: &user_1_name }; // Cannot borrow after transferring ownership
    let user_2 = User { name: &user_2_name };
	let user_3 = User { name: &user_3_name };


	let database:HashMap::<&String, &[u32]>;  // Initialization required
	database.insert(user_1.name, &[1,2,3,4,5]); // Immutable by default
	database.insert(user_2.name, &[42,43,44,45,46,47,48]);

	// Option<T> can be determined to be Some(T) or None
	match database.get(user_1.name) {
		Some(user_data) => {
				println!("Printing {}'s data", user_1.name);
				print_user_data_safe_implementation(user_data, 20)
		}, // Containers are bound-checked
		//None => panic!("User not found"), // Patterns must be exhaustively covered!
	}

	println!("{}", user_3.name);


	// Result<T, Err> can be verified to be Ok(T) or Err
	match database.try_reserve(500) {
		Ok(()) => (),
		Err(_) => panic!("Cannot reserve additional database records"),
	}

}

fn print_user_data_safe_implementation(user_data:&[u32], len:usize){

    for n in 0..len {
        println!("{:?}", user_data[n]);
    }
}

fn print_user_data_unsafe_implementation(user_data:&[u32], len:usize){

    for n in 0..len {
		// Dirty things must be inside an unsafe block
		let raw_ptr = (&user_data[0] as *const u32).add(n);
		println!("{:?}", *raw_ptr);
    }
}

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	#[should_panic]
	fn test_heartbleed() {
		let user_data = [76,32,43,54];
		print_user_data_unsafe_implementation(&user_data, user_data.len() + 1);
	}
}