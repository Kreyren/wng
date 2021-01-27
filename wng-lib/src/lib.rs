#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::io::Write;

    use std::path::Path;

    use toml::Value;

    #[test]
    fn config_reinit() -> super::Result<()> {
        let fname = "test.conf";
        if Path::new(fname).exists() {
            fs::remove_file(fname)?;
        }
        let mut file = fs::File::create(fname)?;
        let content = "cc = \"clang\"\nname = \"Wafelack\"\nemail = \"wafelack@protonmail.com\"";
        file.write_all(
            content.as_bytes()
        )?;

        let mut tomlized = fs::read_to_string(fname)?.parse::<Value>().unwrap();

        assert_eq!(tomlized["cc"].as_str(), Some("clang"));

        super::config::manually::manually(Some(fname), "cc", "gcc")?;

        tomlized = fs::read_to_string(fname)?.parse::<Value>().unwrap();

        assert_eq!(tomlized["cc"].as_str(), Some("gcc"));

        super::config::reinit::reinit(Some(fname))?;

        assert!(&fs::read_to_string(fname)?.is_empty());

        fs::remove_file(fname)?;
        Ok(())

    }

    mod error {

        use super::*;

        #[test]
        fn no_args() {
            assert_eq!(error!(), 
            WngError { line: line!() - 1, file: file!().to_owned(), message: "".to_owned() }
            );
        }
        #[test]
        fn single_messages() {
            assert_eq!(error!("foo"), 
            WngError { line: line!() - 1, file: file!().to_owned(), message: "foo".to_owned() }
            );
        }
        #[test]
        fn multiple_messages() {
            assert_eq!(error!("foo", "bar"), 
            WngError { line: line!() - 1, file: file!().to_owned(), message: "foo bar".to_owned() }
            );
        }
    }
}

pub type Result<T> = std::result::Result<T, WngError>;

impl From<std::io::Error> for WngError {
    fn from(error: std::io::Error) -> Self {
        let msg = format!("{}", error);
        error!(msg)
    }
}

#[derive(PartialEq, Eq)]
pub struct WngError {
    line: u32,
    file: String,
    message: String,
}

impl std::fmt::Display for WngError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {} - {}:{}", self.message, self.file, self.line)
    }
}


impl std::fmt::Debug for WngError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {} - {}:{}", self.message, self.file, self.line)
    }
}

#[macro_export]
macro_rules! error {
    () => {
        WngError {
            line: line!(),
            file: file!().to_string(),
            message: "".to_owned(),
        }
    };
    ($($msg:tt),*) => {
        {
            let mut message = String::new();

            $(
                message.push_str(&format!("{} ", $msg));
            )*

            message.pop();

            WngError {
                line: line!(),
                file: file!().to_string(),
                message,
            }
        }
    };
}


mod config;