use pealn::{import_pea_colors, import_pea_styles, pealn};

import_pea_colors!();
import_pea_styles!();

fn main() {
    let name = "Subham Shaw";
    let age = 2890;
    let phone = "987xxxxx0";
    let email = "johndoe2002@gmail.com";

    let age_color = if age < 18 {
        PeaColor::Red
    } else {
        PeaColor::Green
    };

    pealn!("[yellow,bold](Name)   : [bold,default,purple]({}) ", name);
    pealn!(
        "[yellow,bold](Age)    : [red,{age_color},{PeaStyle::BOLD},underline]({}) ",
        age
    );
    pealn!("[yellow,bold](Phone)  : [orange,bold]({}) ", phone);
    pealn!("[yellow,bold](Email)  : [bold,italic]({}) ", email);
}
