use std::env;

pub fn get_env_var(key: &str) -> String {
    env::var(key).expect(&format!("{} must be set", key))
}

pub fn jwt_secret() -> String {
    get_env_var("JWT_SECRET")
}

pub fn access_exp() -> i64 {
    get_env_var("ACCESS_TOKEN_EXP").parse().unwrap()
}

pub fn refresh_exp() -> i64 {
    get_env_var("REFRESH_TOKEN_EXP").parse().unwrap()
}
