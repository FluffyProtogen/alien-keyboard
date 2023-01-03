use alien_keyboard::{hook, initialize_keys, KEYS};

//ALSO IF CTRL / ALT ARE HELD, DON'T MESS WITH THE KEYS!!!!!!111!1!!!!

fn main() {
    initialize_keys();
    hook::run();
}
