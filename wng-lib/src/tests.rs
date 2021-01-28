#[cfg(test)]
mod test {
    use crate::*;
    use std::fs;
    use std::io::Write;

    use std::path::Path;

    use toml::Value;

    #[test]
    fn config_reinit() -> crate::Result<()> {
        let fname = "./test.conf";
        let mut file = fs::File::create(fname)?;
        let content = "cc = \"clang\"\nname = \"Wafelack\"\nemail = \"wafelack@protonmail.com\"";
        file.write_all(
            content.as_bytes()
        )?;

        let mut tomlized = fs::read_to_string(fname)?.parse::<Value>().unwrap();

        assert_eq!(tomlized["cc"].as_str(), Some("clang"));

        crate::config::manually::manually(Some(fname), "cc", "gcc")?;

        println!("Current dir: {:?}", std::env::current_dir()?);

        let entries = fs::read_dir(".")?;

        println!("Files in current dir: ");

        for entry in entries {
            let entry = entry?;

            println!("{:?}", entry.path());
        }

        println!("File name: {}", fname);

        tomlized = fs::read_to_string(fname)?.parse::<Value>().unwrap();
        
        unimplemented!();

        assert_eq!(tomlized["cc"].as_str(), Some("gcc"));

        crate::config::reinit::reinit(Some(fname))?;

        assert!(&fs::read_to_string(fname)?.is_empty());

        fs::remove_file(fname)?;
        Ok(())

    }



    #[test]
    fn creation_adding() -> Result<()> {
        let fname = "test_creation.conf";
        let mut file = fs::File::create(fname)?;
        let content = "cc = \"clang\"\nname = \"Wafelack\"\nemail = \"wafelack@protonmail.com\"";
        file.write_all(
            content.as_bytes()
        )?;

        crate::create::create("test_creation",Some(fname), false)?;

        std::env::set_current_dir("./test_creation")?;

        crate::deps::add::add_dep("wafelack/wng", true)?;

        std::env::set_current_dir("..")?;

        fs::remove_file(fname)?;

        fs::remove_dir_all("test_creation")?;

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