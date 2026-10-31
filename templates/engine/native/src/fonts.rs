// Exact v9.12 @font-face assets, decompressed from render/fonts.css.
// Family names are CSS aliases; preserve original font names and glyph metrics.
pub const FONT_ASSETS: &[(&str, u16, &[u8])] = &[
    ("Geist", 400, include_bytes!("../assets/fonts/geist-400.ttf")),
    ("Geist", 500, include_bytes!("../assets/fonts/geist-500.ttf")),
    ("Geist", 600, include_bytes!("../assets/fonts/geist-600.ttf")),
    ("Geist", 700, include_bytes!("../assets/fonts/geist-700.ttf")),
    ("Geist", 800, include_bytes!("../assets/fonts/geist-800.ttf")),
    ("Archivo", 400, include_bytes!("../assets/fonts/archivo-400.ttf")),
    ("Archivo", 500, include_bytes!("../assets/fonts/archivo-500.ttf")),
    ("Archivo", 600, include_bytes!("../assets/fonts/archivo-600.ttf")),
    ("Archivo", 700, include_bytes!("../assets/fonts/archivo-700.ttf")),
    ("Archivo", 800, include_bytes!("../assets/fonts/archivo-800.ttf")),
    ("Archivo", 900, include_bytes!("../assets/fonts/archivo-900.ttf")),
    ("Space Grotesk", 400, include_bytes!("../assets/fonts/space-grotesk-400.ttf")),
    ("Space Grotesk", 500, include_bytes!("../assets/fonts/space-grotesk-500.ttf")),
    ("Space Grotesk", 600, include_bytes!("../assets/fonts/space-grotesk-600.ttf")),
    ("Space Grotesk", 700, include_bytes!("../assets/fonts/space-grotesk-700.ttf")),
    ("IBM Plex Sans", 400, include_bytes!("../assets/fonts/ibm-plex-sans-400.ttf")),
    ("IBM Plex Sans", 500, include_bytes!("../assets/fonts/ibm-plex-sans-500.ttf")),
    ("IBM Plex Sans", 600, include_bytes!("../assets/fonts/ibm-plex-sans-600.ttf")),
    ("IBM Plex Sans", 700, include_bytes!("../assets/fonts/ibm-plex-sans-700.ttf")),
];

/// CSS Fonts weight matching over the actual v9.12 @font-face declarations.
/// For example spotLabel requests 650, whose original CSS face is 700, not a new font.
pub fn css_weight(family: &str, requested: u16) -> Option<u16> {
    FONT_ASSETS.iter().filter(|&&(name, _, _)| name == family)
        .map(|&(_, weight, _)| weight)
        .min_by_key(|&weight| {
            if (400..=500).contains(&requested) {
                if weight >= requested && weight <= 500 { (0, weight - requested) }
                else if weight < requested { (1, requested - weight) }
                else { (2, weight - requested) }
            } else if requested < 400 {
                if weight <= requested { (0, requested - weight) } else { (1, weight - requested) }
            } else if weight >= requested { (0, weight - requested) } else { (1, requested - weight) }
        })
}
