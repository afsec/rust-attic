use std::{collections::BTreeMap, fmt::Display, io};

type MenuOption = u8;

#[derive(Debug)]
struct Menu(BTreeMap<MenuOption, (&'static str, ItemMenu)>);

impl Menu {
    pub(crate) fn go(&self, chosen_option_code: u8) {
        if let Some(item) = self.0.get(&chosen_option_code) {
            item.1.go();
        }
    }
}

impl Display for Menu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const CLEAR: &str = "\x1B[2J\x1B[1;1H";
        let mut printed_menu = "".to_string();
        let menu_title = "Main Menu\n=========\n\n";
        printed_menu.push_str(menu_title);
        for (key, data) in self.0.iter() {
            let item = format!("{key}. {name}\n", name = data.0);
            printed_menu.push_str(item.as_str())
        }
        write!(f, "{clear}{menu}", clear = CLEAR, menu = printed_menu)
    }
}

impl<const N: usize> From<[(MenuOption, (&'static str, ItemMenu)); N]> for Menu {
    fn from(mut arr: [(MenuOption, (&'static str, ItemMenu)); N]) -> Self {
        if N == 0 {
            Self(BTreeMap::new())
        } else {
            arr.sort_by(|a, b| a.0.cmp(&b.0));
            Self(BTreeMap::from(arr))
        }
    }
}

trait MenuItem {
    fn go(&self);
}

trait CodeItem {
    fn run(&self)
    where
        Self: std::fmt::Debug,
    {
        use std::io::Write;

        dbg!(&self);

        print!("Press [Enter] to continue...");

        let mut chosen_option = String::new();
        io::stdout().flush().expect("Cannot flush");
        io::stdin()
            .read_line(&mut chosen_option)
            .expect("Failed to read line");
    }
}

#[derive(Debug)]
struct CreateUser;
impl CodeItem for CreateUser {}

#[derive(Debug)]
struct ReadUser;
impl CodeItem for ReadUser {}

#[derive(Debug)]
struct Quit;
impl CodeItem for Quit {}

#[derive(Debug)]
enum ItemMenu {
    // NoOp(NoOp),
    CreateUser(CreateUser),
    ReadUser(ReadUser),
    Quit,
}
impl MenuItem for ItemMenu {
    fn go(&self)
    where
        Self: std::fmt::Debug,
    {
        match self {
            ItemMenu::CreateUser(code) => code.run(),
            ItemMenu::ReadUser(code) => code.run(),
            ItemMenu::Quit => (),
        }
    }
}

fn main() {
    use std::io::Write;

    let data = [
        // (0, ("No operation", ItemMenu::NoOp(NoOp))),
        (1, ("Create User", ItemMenu::CreateUser(CreateUser))),
        (2, ("Read User", ItemMenu::ReadUser(ReadUser))),
        (9, ("Quit", ItemMenu::Quit)),
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
            let chosen_option_code = match chosen_option.trim().parse() {
                Ok(code) => code,
                Err(_) => 0,
            };

            if chosen_option_code > 0 {
                menu.go(chosen_option_code);
                if chosen_option_code == 9 {
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
}
