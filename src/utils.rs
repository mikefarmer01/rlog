use plotters::style::RGBColor;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn to_rgb(arr_rgb: [u8; 3]) -> RGBColor {
    RGBColor(arr_rgb[0], arr_rgb[1], arr_rgb[2])
}

pub fn str_to_rgb(str_rgb: String) -> RGBColor {
    let chars: &[_] = &['r', 'g', 'b', '(', ')'];
    let s = str_rgb.replace(" ", "");
    let vec_color: Vec<u8> = s
        .trim_matches(chars)
        .split(',')
        .map(|s| s.parse::<u8>().expect("Invalid RGB value."))
        .collect();
    vec_to_rgb(vec_color)
}

fn vec_to_rgb(vec_color: Vec<u8>) -> RGBColor {
    RGBColor(vec_color[0], vec_color[1], vec_color[2])
}
