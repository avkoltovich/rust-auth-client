pub mod file_io {
    use std::{io::Write, fs::{self, File}};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct LoginData {
        pub login: String,
        pub password: String,
        pub org_key: String
    }

    pub fn read_from_file(file_name: &str) -> Result<LoginData, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(file_name);
        let login_data: LoginData = serde_json::from_str(&contents.unwrap()[..])?;

        Ok(login_data)
    }

    pub fn store_access_token_to_file(access_token: &String) -> std::io::Result<()> {
        let mut file = File::create("access_token.txt")?;
        let access_token_str = &access_token[..];
        file.write_all(access_token_str.as_bytes())?;

        Ok(())
    }

    pub fn store_refresh_token_to_file(refresh_token: &String) -> std::io::Result<()> {
        let mut file = File::create("refresh_token.txt")?;
        let refresh_token_str = &refresh_token[..];
        file.write_all(refresh_token_str.as_bytes())?;

        Ok(())
    }
}