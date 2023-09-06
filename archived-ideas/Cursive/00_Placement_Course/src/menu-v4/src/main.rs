use crate::{
    applets::{CreateUser, QuitMenu, ReadUser},
    menu::ItemNumber,
};

mod applets;
mod menu;

fn main() -> Result<(), String> {
    menu()
}

fn menu<'label>() -> Result<(), String> {
    use crate::applets::Handler;
    use crate::menu::Menu;
    use std::io::{self, Write};

    let data: [(u8, (String, Box<dyn Handler>)); 3] = [
        (
            1,
            (
                "Create User".to_string(),
                Box::new(CreateUser) as Box<dyn Handler>,
            ),
        ),
        (
            2,
            (
                "Read User".to_string(),
                Box::new(ReadUser) as Box<dyn Handler>,
            ),
        ),
        (
            9,
            ("Quit".to_string(), Box::new(QuitMenu) as Box<dyn Handler>),
        ),
    ];

    let menu = Menu::from(data);
    let mut inside_menu = true;
    while inside_menu {
        println!("{menu}");

        print!("Choose you option: ");

        let mut chosen_option = String::new();
        io::stdout().flush().expect("Cannot flush");
        io::stdin()
            .read_line(&mut chosen_option)
            .expect("Failed to read line");
        if !chosen_option.trim().is_empty() {
            let chosen_option_number: ItemNumber = match chosen_option.trim().parse() {
                Ok(code) => code,
                Err(_) => 0,
            };

            if chosen_option_number > 0 {
                menu.go_item(chosen_option_number)?;
                if chosen_option_number == 9 {
                    inside_menu = false
                }
            } else {
                use std::thread::sleep;
                use std::time::Duration;
                println!("\nPlease type a valid number! [0-9]");
                sleep(Duration::from_secs(1));
            }
        } else {
            use std::thread::sleep;
            use std::time::Duration;
            println!("\nSorry you must choose an option.");
            sleep(Duration::from_secs(1));
        }
    }
    Ok(())
}
