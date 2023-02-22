use key_stroke_counter::{get_config, count_keystrokes};

fn main() {
    let config = get_config();
    
    count_keystrokes(config).unwrap_or_else(|error| panic!("{}", error))
}
