#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn reinit() -> super::Result<()> {
        let fname = "test.conf";
        let mut file = fs::File::create(fname)?;
        let content = "foo\nbar\nmoo";
        file.write_all(content.as_bytes())?;

        assert_eq!(&fs::read_to_string(fname)?, content);

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

#[derive(Debug, PartialEq, Eq)]
struct WngError {
    line: u32,
    file: String,
    message: String,
}

impl std::fmt::Display for WngError {
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