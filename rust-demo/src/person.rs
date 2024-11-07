// todo: 1. 创建标准类
#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    gender: String,
    marital_status: String,
}

impl Person {
    pub fn new(name: String, gender: String, marital_status: String) -> Person {
        Person {
            name: name,
            gender: gender,
            marital_status: marital_status,
        }
    }

    pub fn get_gender(&self) -> String {
        format!("{}", self.gender)
        // self.gender    // todo: 不可以转移所有权， 不能这么写
    }
}

impl Default for Person {
     fn default() -> Self {
        Person {
            name: "Default".to_string(),
            gender: "Default".to_string(),
            marital_status: "Default".to_string(),
        }
    }
}
