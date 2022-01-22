use std::ffi::{CStr, CString, NulError};
use std::os::raw::c_char;
use jq_sys::{
    jq_compile, jq_init, jq_next, jq_start, jv, jv_copy,
    jv_dump_string, jv_parser_new, jv_parser_next, jv_get_kind,
    jv_parser_set_buf, jv_string_value, jv_kind_JV_KIND_INVALID
};

pub enum Error {
    InvalidJSON,
    InvalidProgram,
    SystemError
}

impl From<NulError> for Error {
    fn from(_: NulError) -> Self {
        Error::SystemError
    }
}

pub fn run(program: &str, input: &str) -> Result<String, Error> {
    if input.trim().is_empty() {
        return Ok("".to_string());
    }

    let program = CString::new(program)?;
    let input = CString::new(input)?;

    let jq = unsafe  { jq_init() };
    if jq.is_null() {
        return Err(Error::SystemError)
    }

    if unsafe { jq_compile(jq, program.as_ptr()) } == 0 {
        return Err(Error::InvalidProgram);
    }

    let parser = unsafe { jv_parser_new(0) };
    let parsed_input = unsafe {
        jv_parser_set_buf(
            parser,
            input.as_ptr(),
            input.as_bytes().len() as i32,
            1
        );
        let value = jv_parser_next(parser);
        if is_valid(value) {
            Ok(value)
        } else {
            Err(Error::InvalidJSON)
        }
    }?;


    let mut buf = String::new();
    Ok(unsafe {
        jq_start(jq, jv_copy(parsed_input), 0);

        let mut out = jq_next(jq);
        while is_valid(out) {
            let dump = jv_dump_string(jv_copy(out), 525);
            let s = get_string_value(jv_string_value(dump));
            buf.push_str(&s);
            buf.push('\n');
            out = jq_next(jq);
        }
        // jq_teardown(&mut jq);
        buf
    })
}

unsafe fn get_string_value(value: *const c_char) -> String {
    CStr::from_ptr(value).to_str().unwrap().to_owned()
}

unsafe fn is_valid(msg: jv) -> bool {
    jv_get_kind(msg) != jv_kind_JV_KIND_INVALID
}
