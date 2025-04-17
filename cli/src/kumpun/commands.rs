pub mod check;
pub mod dev;
pub mod docs;
pub mod generate;

pub fn init_all() {
    check::init();
    dev::init();
    docs::init();
    generate::init();
}