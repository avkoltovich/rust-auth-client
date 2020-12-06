pub mod file_io {
    use std::fs;
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
}