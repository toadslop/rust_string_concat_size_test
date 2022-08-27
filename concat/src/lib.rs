use wasm_bindgen::prelude::wasm_bindgen;

const DASH: &'static str = "-";
const COL: &'static str = "col";
const FORM: &'static str = "form";
const LABEL: &'static str = "label";
const LG: &'static str = "lg";
const SM: &'static str = "sm";
const CHECK: &'static str = "check";
const INLINE: &'static str = "inline";
const INPUT: &'static str = "input";
const CONTROL: &'static str = "control";
const COLOR: &'static str = "color";
const FILE: &'static str = "file";
const PLAINTEXT: &'static str = "plaintext";
const RANGE: &'static str = "range";
const FLOATING: &'static str = "floading";
const GROUP: &'static str = "group";
const SELECT: &'static str = "select";
const SWITCH: &'static str = "switch";
const TEXT: &'static str = "text";
const IS: &'static str = "is";
const VALID: &'static str = "valid";
const INVALID: &'static str = "invalid";
const READONLY: &'static str = "readonly";
const FEEDBACK: &'static str = "feedback";
const TOOLTIP: &'static str = "tooltip";

#[wasm_bindgen]
pub fn main() {
    let form = FORM.to_string();
    let col_form_label = [COL, DASH, FORM, DASH, LABEL].concat();
    let col_form_label_lg = [COL, DASH, FORM, DASH, LABEL, DASH, LG].concat();
    let col_form_label_sm = [COL, DASH, FORM, DASH, LABEL, DASH, SM].concat();
    let form_check = [FORM, DASH, CHECK].concat();
    let form_check_inline = [FORM, DASH, CHECK, DASH, INLINE].concat();
    let form_check_input = [FORM, DASH, CHECK, DASH, INPUT].concat();
    let form_check_label = [FORM, DASH, CHECK, DASH, LABEL].concat();
    let form_control = [FORM, DASH, CONTROL].concat();
    let form_control_color = [FORM, DASH, CONTROL, DASH, COLOR].concat();
    let form_control_file = [FORM, DASH, CONTROL, DASH, FILE].concat();
    let form_control_lg = [FORM, DASH, CONTROL, DASH, LG].concat();
    let form_control_plaintext = [FORM, DASH, CONTROL, DASH, PLAINTEXT].concat();
    let form_control_range = [FORM, DASH, CONTROL, DASH, RANGE].concat();
    let form_control_sm = [FORM, DASH, CONTROL, DASH, SM].concat();
    let form_floating = [FORM, DASH, FLOATING].concat();
    let form_group = [FORM, DASH, GROUP].concat();
    let form_inline = [FORM, DASH, INLINE].concat();
    let form_label = [FORM, DASH, LABEL].concat();
    let form_select = [FORM, DASH, SELECT].concat();
    let form_select_lg = [FORM, DASH, SELECT, DASH, LG].concat();
    let form_select_sm = [FORM, DASH, SELECT, DASH, SM].concat();
    let form_switch = [FORM, DASH, SWITCH].concat();
    let form_text = [FORM, DASH, TEXT].concat();
    let input_group_text = [INPUT, DASH, GROUP, DASH, TEXT].concat();
    let is_valid = [IS, DASH, VALID].concat();
    let is_invalid = [IS, DASH, INVALID].concat();
    let readonly = READONLY.to_string();
    let valid_feedback = [VALID, DASH, FEEDBACK].concat();
    let valid_tooltip = [VALID, DASH, TOOLTIP].concat();

    let classes = [
        col_form_label,
        col_form_label_lg,
        col_form_label_sm,
        form,
        form_check,
        form_check_inline,
        form_check_input,
        form_check_label,
        form_control,
        form_control_color,
        form_control_file,
        form_control_lg,
        form_control_plaintext,
        form_control_range,
        form_control_sm,
        form_floating,
        form_group,
        form_inline,
        form_label,
        form_select,
        form_select_lg,
        form_select_sm,
        form_switch,
        form_text,
        input_group_text,
        is_invalid,
        is_valid,
        readonly,
        valid_feedback,
        valid_tooltip,
    ];

    let sum: usize = classes.iter().map(|string| string.len()).sum();
}
