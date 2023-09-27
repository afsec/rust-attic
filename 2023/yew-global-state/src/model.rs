use yew::{html, AttrValue, Callback, Html};

use crate::layout::my_list::item::Item;

/// This is the shared state between the parent and child components.
#[derive(Clone, PartialEq)]
pub struct Model {
    pub vdom: Option<Vlist>,
    pub data: Data,
    /// Callback used when a child is clicked. The AttrValue is the name of the child that was
    /// clicked.
    pub child_clicked: Callback<AttrValue>,
    /// The name of the child that was last clicked.
    pub last_clicked: Option<AttrValue>,
}

#[derive(Clone, PartialEq)]
pub struct Data {
    pub users: Vec<User>,
    /// Total number of clicks received.
    pub total_clicks: u32,
}

#[derive(Clone, PartialEq)]
pub struct User {
    id: u16,
    name: String,
}

impl From<(u16, &'static str)> for User {
    fn from((id, name): (u16, &'static str)) -> Self {
        Self {
            id,
            name: name.to_owned(),
        }
    }

    // fn from(value: &'static str) -> Self {
    //     Self(value.to_owned())
    // }
}
/// This is the shared state between the parent and child components.
#[derive(Clone, PartialEq)]
pub struct Vlist(pub Vec<VItem>);

// impl FromIntoIterator<VItem> for Vlist {
//     fn from_iter<T: IntoIterator<Item = VItem>>(iter: T) -> Self {

//     }
// }
#[derive(Clone, PartialEq)]
pub struct VItem {
    id: u16,
    name: String,
}

impl From<&User> for VItem {
    fn from(user: &User) -> Self {
        Self {
            id: user.id,
            name: user.name.clone(),
        }
    }
}

impl VItem {
    pub fn render(&self) -> Html {
        html! {
            <Item id={self.id.to_string()} name={self.name.clone()} />
        }
    }
}
