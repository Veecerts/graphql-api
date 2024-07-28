pub enum EnvVariables {
    HostAddress,
    HostPort,
    DbHost,
    DbPort,
    DbUsername,
    DbDatabase,
    DbPassword,
    // JwtSecret,
    // JwtExpiresIn,
    // JwtIssuer,
}

pub struct Settings;

impl Settings {
    pub fn expect_env(variable: EnvVariables) -> String {
        let var_name = match variable {
            EnvVariables::HostAddress => "HOST_ADDRESS",
            EnvVariables::HostPort => "HOST_PORT",
            EnvVariables::DbHost => "DB_HOST",
            EnvVariables::DbPort => "DB_PORT",
            EnvVariables::DbUsername => "DB_USERNAME",
            EnvVariables::DbDatabase => "DB_DATABASE",
            EnvVariables::DbPassword => "DB_PASSWORD",
            // EnvVariables::JwtSecret => "JWT_SECRET",
            // EnvVariables::JwtExpiresIn => "JWT_EXPIRES_IN",
            // EnvVariables::JwtIssuer => "JWT_ISSUER",
        };

        std::env::var(var_name).expect(&format!("{} must be set", var_name))
    }
}
