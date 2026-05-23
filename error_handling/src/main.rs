// Alternativ 1
/*
enum Option<T>{ // Definera ett alternativ.
	Some(T),	// Ett värde
	None,	// Inget värde
}
*/

fn divideraOption(täljare: f64, nämnare:f64) -> Option<f64> {
	if nämnare == 0.0 {
		None
	} else {
		Some(täljare / nämnare)
	}
}


// Alternativ 2
/*
enum Result<T, E> { // Definera reslutat typ
	Ok(T),    // Representer ett värde
	Err(E),    // Representerar error
}
*/

fn divideraResult(täljare: f64, nämnare:f64) -> Result<f64, String> {
	if nämnare == 0.0 {
		Err("Can't divide by 0".to_string())
	} else {
		Ok(täljare / nämnare)
	}
}

fn main() {
	
	// Dividera med Option error hantering
	let resultat_Option = divideraOption(10.0, 0.0);
	match resultat_Option {
		Some(resultat)=> println!("Result: {resultat}"),
		None => println!("Error, can't divide by zero!"),
	}

	// Dividera mer Result error hantering
	let resultat_Result = divideraResult(100.23, 73.98);
	match resultat_Result {
		Ok(result) => println!("Result: {result}"),
		Err(err) => println!("Error: {err}"),
	}
	
}
