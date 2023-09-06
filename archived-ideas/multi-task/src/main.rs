use std::{thread, time};


fn iniciar_corrida(x: i32) {
    print!("{}:",x);

	let carro_a = thread::spawn(move || {
        for i in 1..11 {
            println!("{}","A".repeat(i));

        }
	});
	
	let carro_b = thread::spawn(move || {
        for i in 1..11 {
            println!("{}","B".repeat(i));
        }
	});

	let carro_c = thread::spawn(move || {
        for i in 1..11 {
            println!("{}","C".repeat(i));
        }
	});
	
    let carro_d = thread::spawn(move || {
        for i in 1..11 {
            println!("{}","D".repeat(i));
        }
	});

    let carro_e = thread::spawn(move || {
        for i in 1..11 {
            println!("{}","E".repeat(i));
        }
	});

	carro_a.join().unwrap();
	carro_b.join().unwrap();
	carro_c.join().unwrap();
	carro_d.join().unwrap();
	carro_e.join().unwrap();
}


fn main() {
    let delay = time::Duration::from_millis(500);
    for volta in 0..3 {
        iniciar_corrida(volta);
        println!("");
        thread::sleep(delay);
    }
}

