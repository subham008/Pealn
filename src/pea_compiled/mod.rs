use regex::Regex;

pub mod multi_value;
pub mod pea_code_block;
pub mod pea_color;
pub mod pea_modifier;
pub mod pea_styles;

use multi_value::MultiValue;
use pea_code_block::PeaCodeBlock;
use pea_color::PeaColor;
use pea_modifier::PeaModifier;
use pea_styles::PeaStyle;

#[derive(Debug, Clone)]
pub struct PeaCompiled {
    pub foreground: MultiValue<Option<PeaColor>, PeaCodeBlock>, // RGB values for foreground color
    pub background: MultiValue<Option<PeaColor>, PeaCodeBlock>, // RGB values for background color
    pub styles: Vec<MultiValue<PeaStyle, PeaCodeBlock>>,        // all styles applied
}

enum PealnError {
    InvalidArgument,
    RepeatedColor,
    RepeatedStyle,
}

fn panic_pealn_error(error: PealnError, arg: &str, code: &str) {
    match error {
        PealnError::InvalidArgument => {
            panic!("pealn error : invalid argument ` {} ` at {}", arg, code);
        }
        PealnError::RepeatedColor => {
            panic!(
                "pealn error : repeated color ` {} ` at {} \n  You may have used more than 2 colors , you can use only one foreground and one background color",
                arg, code
            );
        }
        PealnError::RepeatedStyle => {
            panic!(
                "pealn error : repeated style ` {} ` at {} \n  You may have used same style more than once",
                arg, code
            );
        }
    }
}

impl PeaCompiled {
    pub fn from_modifier(raw_modifier: &String, full_code: &String) -> Self {
        //now modifier will be compiled to get colors, styles and code block

        //here we will use regex to split the modifier string by commas, but ignore commas inside parentheses
        let re = Regex::new(r"\((?:[^\(\)]|(?R))*\)|[^,\[\]\s][^,\[\]]*").unwrap();
        let args: Vec<String> = re
            .find_iter(raw_modifier)
            .map(|mat| mat.as_str().trim().to_string())
            .collect();

        // Process the args to extract foreground, background, and styles
        let mut foreground = MultiValue::First(None);
        let mut background = MultiValue::First(None);
        let mut styles: Vec<MultiValue<PeaStyle, PeaCodeBlock>> = Vec::new();

        //firt two argumeny are colors, then styles :  [foreground, background, styles...] if something like link is found it will be called as modifier
        for arg in args {
            //identifying type of modifier

            // Check for color first
            let modifier: PeaModifier = if let Some(color) = PeaColor::from(arg.as_str()) {
                PeaModifier::Color(color)
            }
            // Then check for style
            else if let Some(style) = PeaStyle::from(arg.as_str()) {
                PeaModifier::Style(style)
            }
            // Then check for code block
            else if let Some(code_block) = PeaCodeBlock::from(arg.clone()) {
                PeaModifier::CodeBlock(code_block)
            } else {
                panic_pealn_error(PealnError::InvalidArgument, &arg, &full_code);
                continue;
            };

            match modifier {
                PeaModifier::Color(color) => {
                    if foreground.is_first() && foreground.first().unwrap().is_none() {
                        foreground = MultiValue::First(Some(color));
                    } else if background.is_first() && background.first().unwrap().is_none() {
                        background = MultiValue::First(Some(color));
                    } else {
                        panic_pealn_error(PealnError::RepeatedColor, &arg, &full_code);
                    }
                }

                PeaModifier::Style(style) => {
                    //hereve we have to check if style is already present in styles vector
                    if styles
                        .iter()
                        .any(|s| s.is_first() && s.first().unwrap() == &style)
                    {
                        panic_pealn_error(PealnError::RepeatedStyle, &arg, &full_code);
                        continue;
                    }else{
                    styles.push(MultiValue::First(style));
                    }
                }
                PeaModifier::CodeBlock(code_block) => {
                    if foreground.is_first() && foreground.first().unwrap().is_none() {
                        foreground = MultiValue::Second(code_block);
                    } else if background.is_first() && background.first().unwrap().is_none() {
                        background = MultiValue::Second(code_block);
                    } else {
                        styles.push(MultiValue::Second(code_block));
                    }
                }
            }
        } //end of loop

        PeaCompiled {
            foreground: foreground,
            background: background,
            styles: styles,
        }
    }
}
