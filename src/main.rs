mod makepig;
use makepig::*;

fn main() {
    let pig = make_pig();
    let pigtext = pig.text_to_pig_latin("Can't touch this! Awoo-away!");
    println!("{}", pigtext);
}
