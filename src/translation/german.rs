use phf::phf_map;

pub static TRANSLATION_TABLE : phf::Map<&str, &str> = phf_map!{
    "Factor:" => "Faktor:",
    "Language" => "Sprache",
    "Settings" => "Einstellungen",
    "Cake Size Converter" => "Kuchengrößen Rechner",
    "Diameter" => "Durchmesser",
    "Height" => "Höhe",
    "Length" => "Länge",
    "Width" => "Breite",
    "Inner Diameter" => "Innendurchmesser",
    "Outer Diameter" => "Außendurchmesser",
    "Custom" => "Anders",
    "Volume" => "Volumen",
    "Round" => "Rund",
    "Ring" => "Ring",
    "Rectangle" => "Kasten",
    "Shape" => "Form",
    "Same Height" => "Höhe beibehalten"
};