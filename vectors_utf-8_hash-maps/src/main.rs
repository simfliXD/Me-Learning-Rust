#![allow(warnings)]

use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    vectors();
    utf8();
    hash_maps();
}

fn vectors(){
	let mut _v: Vec<i32> = Vec::new(); // Skapa en vector

	let mut _v:Vec<i32> = vec![1,2,3]; // Macro för det nedan

	_v.push(4);
	_v.push(5);
	_v.push(6);
	_v.push(7);
	_v.push(8);

	println!("Vector v: {:?}", _v);


	let _vector = vec![1,2,3,4,5,6];

	let third: &i32 = &_v[2]; // index

	println!("The third element of Vector is {third}");

	let fourth = _vector.get(3);
	match fourth {
		Some(fourth) =>  println!("The fouth element for a GET method is {fourth}"),
		None => println!("There exists no fourth element."),
	}
}

fn utf8() {
	// 1
	let s = "somthing".to_string();
	// 2
	let s = String::from("String text");
	// 3 Mutate the variable (Push to it)
	let mut s = String::from("food");
	s.push_str("bar");
	s.push('!');

	println!("The value of s = {s}");

	let s1 = String::from("Hello, ");
	let s2 = String::from("World!");
	let s3 = s1 + &s2; // s1 blev dropped
	println!("Value of s3 = {s3}");
	
	let firstname = String::from("Иван");
	let lastname = String::from("Åberg");
	let fullname = format!("{firstname} {lastname}");
	println!("Full name: {}", fullname);
}

fn hash_maps(){
	let mut scores = std::collections::HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Red"), 50);

	let team_name = String::from("Blue");
	let score = scores.get(&team_name).copied().unwrap_or(0);
	
	for (key, value) in &scores {
		println!("{key}: {value}");
	}
}