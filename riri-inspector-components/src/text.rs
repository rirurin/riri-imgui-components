use imgui;

pub fn create_multiline_text<T>(ui: &mut imgui::Ui, text: T) 
where T: AsRef<str>
{
    let font_id = *ui.fonts().fonts().first().unwrap();
    let font  = ui.fonts().get_font(font_id).unwrap();
    // implement word wrap
    let word_boundaries: Vec<usize> = text.as_ref().match_indices(" ").map(|v| v.0).collect();
    let mut fpos = 0;
    while fpos < text.as_ref().len() {
        let line_len = (ui.content_region_avail()[0] / font.fallback_advance_x) as usize;
        let start = fpos;
        fpos = (fpos + line_len).min(text.as_ref().len());
        // Find the first word boundary that exceeds this
        if let Some(bound_index) = word_boundaries.iter().position(|p| *p > fpos) {
            // go to the word before that, but include the space
            fpos = word_boundaries[bound_index - 1] + 1;
        }
        ui.text(&text.as_ref()[start..fpos]);
    }
}