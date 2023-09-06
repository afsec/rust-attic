use std::{collections::BTreeMap, fmt::Display};

use crate::applets::Handler;

pub(in crate) struct Menu(BTreeMap<ItemNumber, ItemData>);

impl Menu {
    pub fn go_item(&self, item_number: ItemNumber) -> Result<(), String> {
        if let Some(item) = self.0.get(&item_number) {
            item.1.handler();
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
            let item = format!("{number}. {label}\n", number = data.0, label = data.1 .0);
            printed_menu.push_str(item.as_str())
        }
        write!(f, "{clear}{menu}", clear = CLEAR, menu = printed_menu)
    }
}
pub(in crate) type ItemNumber = u8;

pub(in crate) type ItemData = (ItemLabel, ItemCode);

pub(in crate) type ItemLabel = String;
pub(in crate) type ItemCode = Box<dyn Handler>;
