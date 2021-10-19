use std::fmt;

struct List(Vec<i32>);

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{0:.2$} + {1:.2$}i", self.real, self.imag, 2)
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	let vec = &self.0;	// extacts the value using tuple indexing
	write!(f, "[")?;		// writes the opening bracket for completed string
	for (index, v) in vec.iter().enumerate() {
	    if index != 0 { write!(f, ", ")?; }
	    write!(f, "{}:{}", index, v)?;
	}
	write!(f, "]")
    }
}

fn main() {

    let complex = Complex {real: 3.3, imag: 7.2};

    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    let v = List(vec![1,2,3]);
    println!("{}", v);
}
