#[no_mangle]
extern "C" fn help() {
    println!("usage hello!!!!!!");
}

#[no_mangle]
extern "C" fn presenter(input: Option<String>) -> String {
    let model = model(input);
    let view = view(model);
    view
}

fn model(input: Option<String>) -> Result<String, String> {
    match input {
        Some(data) => Ok(data),
        None => Err("Wrong input data!".to_string())
    }
}

fn view(msg: Result<String, String>) -> String {
    match msg {
        Ok(data) => {
            let template = format!("{}", data);
            template
        }
        Err(error) => {
            let template = format!(
                r#"
             ____________
            < {} >
             ------------
                    \   ^__^
                     \  (oo)\_______
                        (__)\       )\/\
                            ||----w |
                            ||     ||
            "#,
                error
            );
            template
        }
    }
}
