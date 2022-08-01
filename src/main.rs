use std::io;

fn main() {
    println!("Please enter a number to start temperature conversion.");
    let mut tempinput = String::new();
    io::stdin()
        .read_line(&mut tempinput)
        .expect("Please enter a number!!!");

    let tempinput: f64 = tempinput.trim().parse().expect("Please enter a number!!!");

    println!("Do you want to convert from Celcius or Fahrenheit? c/f");

    let mut c_or_f = String::new();

    let fahrenheit = String::from("f");

    let celcius = String::from("c");

    io::stdin()
        .read_line(&mut c_or_f)
        .expect("Failed to read line");
    let c_or_f = c_or_f.trim();
    if c_or_f == celcius {
        println!("Converting from celcius to fahrenheit!");
        let tempinput = tempinput * 1.8 + 32.0;
        println!("Fahrenheit value: {tempinput}");
    } else if c_or_f == fahrenheit {
        println!("Converting from fahrenheit to celcius!");
        let tempinput = (tempinput - 32.0) * 0.5556;
        println!("Celcius value: {tempinput}");
    } else {
        panic!("Please enter c or f.");
    }
}
