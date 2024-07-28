pub enum EnvVariables {
    HostAddress,
    Port,
    DatabaseUrl,
    PinyataApiKey,
    PinyataApiSecret,
    PinyataJwt,
}

pub struct Settings;

impl Settings {
    pub fn expect_env(variable: EnvVariables) -> String {
        let var_name = match variable {
            EnvVariables::HostAddress => "HOST_ADDRESS",
            EnvVariables::Port => "PORT",
            EnvVariables::DatabaseUrl => "DATABASE_URL",
            EnvVariables::PinyataApiKey => "PINYATA_API_KEY",
            EnvVariables::PinyataApiSecret => "PINYATA_API_SECRET",
            EnvVariables::PinyataJwt => "PINYATA_JWT",
        };

        std::env::var(var_name).expect(&format!("{} must be set", var_name))
    }
}
