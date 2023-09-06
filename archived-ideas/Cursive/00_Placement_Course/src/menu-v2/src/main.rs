use std::{collections::BTreeMap, fmt::Display};

#[derive(Debug)]
struct Menu(BTreeMap<u8, (&'static str, MenuItems)>);
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

impl From<BTreeMap<u8, (&'static str, MenuItems)>> for Menu {
    fn from(data: BTreeMap<u8, (&'static str, MenuItems)>) -> Self {
        Self(data)
    }
}

trait MenuItem {
    fn go(&self)
    where
        Self: std::fmt::Debug,
    {
        dbg!(&self);
    }
}

#[derive(Debug)]
struct NoOp;
impl MenuItem for NoOp {}

#[derive(Debug)]
struct CreateUser;
impl MenuItem for CreateUser {}

#[derive(Debug)]
struct ReadUser;
impl MenuItem for ReadUser {}

#[derive(Debug)]
struct Quit;
impl MenuItem for Quit {}

#[derive(Debug)]
enum MenuItems {
    NoOp(NoOp),
    CreateUser,
    ReadUser,
    Quit,
}

fn main() {
    let data = [
        (0, ("No operation", MenuItems::NoOp(NoOp))),
        (1, ("Create User", MenuItems::CreateUser)),
        (2, ("Read User", MenuItems::ReadUser)),
        (9, ("Quit", MenuItems::Quit)),
    ];
    let menu = Menu::from(BTreeMap::from(data));
    println!("{menu}");
}
