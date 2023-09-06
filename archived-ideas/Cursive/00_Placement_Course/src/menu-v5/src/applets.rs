use std::thread::sleep;
use std::time::Duration;

pub(in crate) const DELAY: Duration = Duration::from_secs(2);

pub(in crate) trait Handler {
    fn handler(&self);
}

#[derive(Debug)]
pub(in crate) struct CreateUser;
impl Handler for CreateUser {
    fn handler(&self) {
        println!("");
        println!("Create user");
        sleep(DELAY)
    }
}

#[derive(Debug)]
pub(in crate) struct ReadUser;
impl Handler for ReadUser {
    fn handler(&self) {
        println!("");
        println!("Read user");
        sleep(DELAY)
    }
}

#[derive(Debug)]
pub(in crate) struct QuitMenu;
impl Handler for QuitMenu {
    fn handler(&self) {
        println!("");
        println!("Quiting...");
        sleep(DELAY)
    }
}
