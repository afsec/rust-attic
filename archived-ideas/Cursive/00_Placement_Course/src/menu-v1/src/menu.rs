use std::{collections::BTreeMap, fmt::Display};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

pub(in crate) type ItemCode = u8;
pub(in crate) type ItemName = &'static str;

#[derive(Debug)]
pub(in crate) struct MainMenu(BTreeMap<ItemCode, ItemName>);

impl MainMenu {
    pub(in crate) fn new() -> Self {
        let mut menu = BTreeMap::new();

        // * No operation
        {
            let (item_code, menu_item) = MenuItemNoOp::new().take();
            menu.insert(item_code, menu_item);
        }

        // * Create User
        {
            let (item_code, menu_item) = MenuItemCreateUser::new().take();
            menu.insert(item_code, menu_item);
        }

        // * Read User
        {
            let (item_code, menu_item) = MenuItemReadUser::new().take();
            menu.insert(item_code, menu_item);
        }

        // * Admin page
        {
            let (item_code, menu_item) = MenuItemAdmin::new().take();
            menu.insert(item_code, menu_item);
        }

        // * Quit
        {
            let (item_code, menu_item) = MenuItemQuit::new().take();
            menu.insert(item_code, menu_item);
        }

        Self(menu)
    }
}

impl Display for MainMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut printed_menu = "".to_string();
        let menu_title = "Main Menu\n=========\n\n";
        printed_menu.push_str(menu_title);
        for (key, value) in self.0.iter() {
            let item = format!("{key}. {value}\n");
            printed_menu.push_str(item.as_str())
        }
        write!(f, "{clear}{menu}", clear = CLEAR, menu = printed_menu)
    }
}

#[derive(Debug)]
pub(in crate) struct MenuItemNoOp((ItemCode, ItemName));
impl MenuItemNoOp {
    pub(in crate) fn new() -> Self {
        Self((0, "No operation"))
    }
    pub(in crate) fn take(self) -> (ItemCode, ItemName) {
        (self.0 .0, self.0 .1)
    }
}

#[derive(Debug)]
pub(in crate) struct MenuItemCreateUser((ItemCode, ItemName));
impl MenuItemCreateUser {
    pub(in crate) fn new() -> Self {
        Self((1, "Create User"))
    }
    pub(in crate) fn take(self) -> (ItemCode, ItemName) {
        (self.0 .0, self.0 .1)
    }
}

#[derive(Debug)]
pub(in crate) struct MenuItemReadUser((ItemCode, ItemName));
impl MenuItemReadUser {
    pub(in crate) fn new() -> Self {
        Self((2, "Read User"))
    }
    pub(in crate) fn take(self) -> (ItemCode, ItemName) {
        (self.0 .0, self.0 .1)
    }
}

#[derive(Debug)]
pub(in crate) struct MenuItemAdmin((ItemCode, ItemName));
impl MenuItemAdmin {
    pub(in crate) fn new() -> Self {
        Self((8, "Admin page"))
    }
    pub(in crate) fn take(self) -> (ItemCode, ItemName) {
        (self.0 .0, self.0 .1)
    }
}

#[derive(Debug)]
pub(in crate) struct MenuItemQuit((ItemCode, ItemName));
impl MenuItemQuit {
    pub(in crate) fn new() -> Self {
        Self((9, "Quit"))
    }
    pub(in crate) fn take(self) -> (ItemCode, ItemName) {
        (self.0 .0, self.0 .1)
    }
}
