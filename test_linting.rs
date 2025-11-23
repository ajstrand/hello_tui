fn main() {
    let name = "world".to_string();
    let result = name.parse::<i32>().unwrap();    // This should trigger an unwrap() lint
    println!("Hello, {}!", result);
    	let x = 5;	// Mixed indentation (tabs and spaces)
    println!("Value: {}", x)
    console.log("This is JavaScript in a Rust file");  // Wrong language
    if result == 5 {  // Should suggest using `=` instead of `==` for assignment
        println!("Equal!");
    }
    // This line has trailing whitespace         
}

// Very long line that exceeds 100 characters: This is a very long comment that should trigger the long line linting rule because it goes beyond the recommended line length limit
