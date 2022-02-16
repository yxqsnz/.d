use pam;
mod interface;
mod login;
fn main() {
    interface::interface::init_display_manager_interface();
}
