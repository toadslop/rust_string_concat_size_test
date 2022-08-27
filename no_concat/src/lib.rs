use wasm_bindgen::prelude::wasm_bindgen;

const COL_FORM_LABEL: &'static str = "col-form-label";
const COL_FORM_LABEL_LG: &'static str = "col-form-label-lg";
const COL_FORM_LABEL_SM: &'static str = "col-form-label-sm";
const FORM: &'static str = "form";
const FORM_CHECK: &'static str = "form-check";
const FORM_CHECK_INLINE: &'static str = "form-check-inline";
const FORM_CHECK_INPUT: &'static str = "form-check-input";
const FORM_CHECK_LABEL: &'static str = "form-check-label";
const FORM_CONTROL: &'static str = "form-control";
const FORM_CONTROL_COLOR: &'static str = "form-control-color";
const FORM_CONTROL_FILE: &'static str = "form-control-file";
const FORM_CONTROL_LG: &'static str = "form-control-lg";
const FORM_CONTROL_PLAINTEXT: &'static str = "form-control-plaintext";
const FORM_CONTROL_RANGE: &'static str = "form-control-range";
const FORM_CONTROL_SM: &'static str = "form-control-sm";
const FORM_FLOATING: &'static str = "form-floating";
const FORM_GROUP: &'static str = "form-group";
const FORM_INLINE: &'static str = "form-inline";
const FORM_LABEL: &'static str = "form-label";
const FORM_SELECT: &'static str = "form-select";
const FORM_SELECT_LG: &'static str = "form-select-lg";
const FORM_SELECT_SM: &'static str = "form-select-sm";
const FORM_SWITCH: &'static str = "form-switch";
const FORM_TEXT: &'static str = "form-text";
const INPUT_GROUP_TEXT: &'static str = "input-group-text";
const IS_INVALID: &'static str = "is-invalid";
const IS_VALID: &'static str = "is-valid";
const READONLY: &'static str = "readonly";
const VALID_FEEDBACK: &'static str = "valid-feedback";
const VALID_TOOLTIP: &'static str = "valid-tooltip";

#[wasm_bindgen]
#[allow(dead_code)]
pub fn main() {
    let classes = [
        COL_FORM_LABEL,
        COL_FORM_LABEL_LG,
        COL_FORM_LABEL_SM,
        FORM,
        FORM_CHECK,
        FORM_CHECK_INLINE,
        FORM_CHECK_INPUT,
        FORM_CHECK_LABEL,
        FORM_CONTROL,
        FORM_CONTROL_COLOR,
        FORM_CONTROL_FILE,
        FORM_CONTROL_LG,
        FORM_CONTROL_PLAINTEXT,
        FORM_CONTROL_RANGE,
        FORM_CONTROL_SM,
        FORM_FLOATING,
        FORM_GROUP,
        FORM_INLINE,
        FORM_LABEL,
        FORM_SELECT,
        FORM_SELECT_LG,
        FORM_SELECT_SM,
        FORM_SWITCH,
        FORM_TEXT,
        INPUT_GROUP_TEXT,
        IS_INVALID,
        IS_VALID,
        READONLY,
        VALID_FEEDBACK,
        VALID_TOOLTIP,
    ];

    let sum: usize = classes.iter().map(|string| string.len()).sum();
}
