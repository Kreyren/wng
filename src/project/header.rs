use std::fs::File;
use std::io::Write;

/// Creates a header file
pub fn header(name: &str) -> Result<(), String> {
    let mut filename = String::from(name);
    filename.push_str(".h");

    let mut ifndef = String::from("#ifndef _");
    ifndef.push_str(&name.to_uppercase());
    ifndef.push_str("_H_\n");

    let mut define = String::from("#define _");
    define.push_str(&name.to_uppercase());
    define.push_str("_H_\n\n\n\n");

    let mut endif = String::from("#endif /* _");
    endif.push_str(&name.to_uppercase());
    endif.push_str("_H_ */\n");

    let mut fic = match File::create(filename) {
        Ok(f) => f,
        Err(e) => return Err(format!("{}", e)),
    };
    match fic.write_all(ifndef.as_bytes()) {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    }
    match fic.write_all(define.as_bytes()) {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    }
    match fic.write_all(endif.as_bytes()) {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    }

    Ok(())
}
