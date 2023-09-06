#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::mem_forget,
    clippy::unused_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::await_holding_lock,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::fn_params_excessive_bools,
    clippy::exit,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::verbose_file_reads,
    clippy::unnested_or_patterns,
    clippy::str_to_string,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    // missing_debug_implementations,
    // missing_docs
)]
#![deny(unreachable_pub, private_in_public)]
#![allow(elided_lifetimes_in_paths, clippy::type_complexity)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]

use crate::{
    applets::{CreateUser, QuitMenu, ReadUser},
    menu::{ItemData, ItemHandler, ItemLabel, ItemNumber},
};

mod applets;
mod menu;

fn main() -> Result<(), String> {
    use crate::applets::Handler;
    use crate::menu::Menu;
    use std::io::{self, Write};

    let data: [(ItemNumber, ItemData); 3] = [
        (
            ItemNumber::from(1),
            ItemData::from((
                ItemLabel::from("Create User".to_string()),
                ItemHandler::from(Box::new(CreateUser) as Box<dyn Handler>),
            )),
        ),
        (
            ItemNumber::from(2),
            ItemData::from((
                ItemLabel::from("Read User".to_string()),
                ItemHandler::from(Box::new(ReadUser) as Box<dyn Handler>),
            )),
        ),
        (
            ItemNumber::from(9),
            ItemData::from((
                ItemLabel::from("Quit".to_string()),
                ItemHandler::from(Box::new(QuitMenu) as Box<dyn Handler>),
            )),
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
            let chosen_option_number: ItemNumber = match chosen_option.trim().parse::<u8>() {
                Ok(number) => ItemNumber::from(number),
                Err(_) => ItemNumber::from(0),
            };

            if &*chosen_option_number > &0 {
                menu.go_item(&chosen_option_number)?;
                if &*chosen_option_number == &9 {
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
