pub mod file_io {
    use std::{io::Write, fs::{self, File}};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct LoginData {
        pub login: String,
        pub password: String,
        pub org_key: String
    }

    pub fn get_login_data_from_file() -> Result<LoginData, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string("login_data.json")?;
        let login_data: LoginData = serde_json::from_str(&contents[..])?;

        Ok(login_data)
    }

    pub fn get_access_token_from_file() -> Result<String, Box<dyn std::error::Error>> {
        let access_token = fs::read_to_string("access_token.txt")?;
        Ok(access_token)
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