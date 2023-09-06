use std::{collections::BTreeMap, fmt::Display, ops::Deref};

use crate::applets::{Handler, DELAY};

pub(in crate) struct Menu(BTreeMap<ItemNumber, ItemData>);

impl Menu {
    pub(in crate) fn go_item(&self, item_number: &ItemNumber) -> Result<(), String> {
        if let Some(item) = self.0.get(item_number) {
            item.handler.handler();
        } else {
            println!("");
            println!("Wrong option!");
            std::thread::sleep(DELAY)
        }

        Ok(())
    }
}

impl<'handler, const N: usize> From<[(ItemNumber, ItemData); N]> for Menu {
    fn from(mut arr: [(ItemNumber, ItemData); N]) -> Self {
        if N == 0 {
            Self(BTreeMap::new())
        } else {
            arr.sort_by(|a, b| a.0.cmp(&b.0));
            Self(BTreeMap::from(arr))
        }
    }
}

impl Display for Menu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const CLEAR: &str = "\x1B[2J\x1B[1;1H";
        let mut printed_menu = "".to_string();
        let menu_title = "Main Menu\n=========\n\n";
        printed_menu.push_str(menu_title);
        for data in self.0.iter() {
            let item = format!("{number}. {label}\n", number = data.0, label = data.1.label);
            printed_menu.push_str(item.as_str())
        }
        write!(f, "{clear}{menu}", clear = CLEAR, menu = printed_menu)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(in crate) struct ItemNumber(u8);

impl Deref for ItemNumber {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ItemNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u8> for ItemNumber {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

pub(in crate) struct ItemData {
    label: ItemLabel,
    handler: ItemHandler,
}

impl From<(ItemLabel, ItemHandler)> for ItemData {
    fn from((label, handler): (ItemLabel, ItemHandler)) -> Self {
        Self { label, handler }
    }
}

pub(in crate) struct ItemLabel(String);
impl Display for ItemLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<String> for ItemLabel {
    fn from(label: String) -> Self {
        Self(label)
    }
}

pub(in crate) struct ItemHandler(Box<dyn Handler>);
impl Handler for ItemHandler {
    fn handler(&self) {
        self.0.handler();
    }
}
impl From<Box<dyn Handler>> for ItemHandler {
    fn from(handler: Box<dyn Handler>) -> Self {
        Self(handler)
    }
}
