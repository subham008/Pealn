use crate::pea_compiled::PeaCodeBlock;
use crate::pea_compiled::PeaColor;
use crate::pea_compiled::PeaStyle;

pub enum PeaModifier {
    Color(PeaColor),
    Style(PeaStyle),
    CodeBlock(PeaCodeBlock),
}
