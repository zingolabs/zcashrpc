//This mod is really really small right now, but I expect it to grow rapidly
#[derive(Debug)]
pub(crate) enum Case {
    Regular,
    FourXs,
    AlsoStandaloneEnum(proc_macro2::TokenStream),
}

pub(crate) const RESERVED_KEYWORDS: &[&str] = &[
    "as", "use", "break", "const", "continue", "crate", "else", "if", "enum",
    "extern", "false", "fn", "for", "if", "impl", "in", "for", "let", "loop",
    "match", "mod", "move", "mut", "pub", "impl", "ref", "return", "Self",
    "self", "static", "struct", "super", "trait", "true", "type", "unsafe",
    "use", "where", "while", "abstract", "alignof", "become", "box", "do",
    "final", "macro", "offsetof", "override", "priv", "proc", "pure", "sizeof",
    "typeof", "unsized", "virtual", "yield",
];
