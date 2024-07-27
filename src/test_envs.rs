#[derive(Clone)]
struct Pair {
    key: String,
    value: Option<String>,
}

pub struct TestEnvs {
    envs: Vec<Pair>,
}

impl TestEnvs {
    fn make_key(key: &str) -> String {
        let id = std::thread::current().id();
        // let id_as_string = format!("{:?}", id).to_uppercase();
        // let id_as_string = id_as_string.replace("(", "_").replace(")", "");
        // format!("{}_{}", id_as_string.to_uppercase(), key)
        key.to_string()
    }

    pub fn get_env(key: &str) -> Option<String> {
        let key = Self::make_key(key);
        std::env::var(key).map_or_else(|_| None, Some)
    }

    pub fn set_var(key: &str, value: &str) {
        let key = Self::make_key(key);
        std::env::set_var(key.as_str(), value);
    }
    pub fn remove_var(key: &str) {
        let key = Self::make_key(key);
        std::env::remove_var(key.as_str());
    }

    fn backup_and_remove_envs(envs: Vec<&str>) -> Vec<Pair> {
        let mut list = vec![];
        for key in envs {
            let value = Self::get_env(key);
            Self::remove_var(key);

            let pair = Pair {
                key: key.to_string(),
                value,
            };
            list.push(pair);
        }
        list
    }

    fn set_env(pair: &Pair) {
        if let Some(value) = pair.value.clone() {
            Self::set_var(pair.key.as_str(), value.as_str());
        } else {
            Self::remove_var(pair.key.as_str());
        }
    }
    fn restore_envs(&self) {
        for pair in &self.envs {
            Self::set_env(pair);
        }
    }

    pub fn new(envs: Vec<&str>) -> Self {
        Self::set_var("IN_USE", "1");
        Self {
            envs: Self::backup_and_remove_envs(envs),
        }
    }

    pub fn make_prefix(prefix: &str) -> String {
        if Self::get_env("IN_USE").is_some() {
            Self::make_key(prefix)
        } else {
            prefix.to_string()
        }
    }
}

impl Drop for TestEnvs {
    fn drop(&mut self) {
        self.restore_envs();
        Self::remove_var("IN_USE");
    }
}
