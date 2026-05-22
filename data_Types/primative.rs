// Primativa data typer
// int, float, bool, char
// 
// Integer
// Kan tillgelas att vara + och - (Signerade)
// eller bara + (Osignerade)
// i8, i16, i32, i64, i128: Signerade (+ och -)
// u8, u16, u32, u64, u128: Osignerade ( + )
fn main(){
	let x: i32 = -42;
	let y: u64 = 100;
	println!("Signerat nummer: {}",x);
	println!("Osignerat nummer: {}",y);
// Differansen mellan i32 (32 bitar) och i64 (64 bitar)
// 
// bredd:
// åt vadera håll (+/-)
// i32 är 32 bitar (2^32) - 2147483648
// i64 är 64 bitar (2^64) - 9223372036854775808
	let i32_max:i32 = 2147483647;
	let i64_max:i64 = 9223372036854775807;

	let i32_min:i32 = -2147483648;
	let i64_min:i64 = -9223372036854775808;
	println!("Max i32: {}",i32_max);
	println!("Max i64: {}",i64_max);
	println!("Min i32: {}",i32_min);
	println!("Min i64: {}",i64_min);

// Floats [Floating Pint Types]
// f32, f64
	let pi: f64 = 3.141592653589793;
	println!("Pi: {}",pi);

// Boolean (Sant/Falskt)
	let is_true: bool = true;
	let is_false: bool = false;
	println!("Is true: {}",is_true);
	println!("Is false: {}",is_false);

// Charachter Type - char (Unicode) - Ett tecken
	let letter: char = 'a';
	let emoji: char = '😀';
	println!("Letter: {}",letter);
	println!("Emoji: {}",emoji);
}