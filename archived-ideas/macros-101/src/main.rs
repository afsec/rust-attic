macro_rules! add {
    ($var:ident,$a:expr,$b:expr) => {
        let x = $a;
        let y = $b;
        let outcome = x + y;
        // let outcome = format!("{}", (x + y));
        println!("1. Outcome (inside): {:?}", $var);
        $var = Some(outcome);
        println!("2. Outcome (inside): {:?}", $var);
    };
}
// type MyData = Option<String>;
type MyData = Option<i32>;

fn main() {
    let data: MyData = Default::default();
    run(data);
    println!("4. main()::data: {:?}", data);
}

fn run(mut data: MyData) {
    add!(data, 1, 2);
    println!("3. run()::Data: {:?}", data);
}
