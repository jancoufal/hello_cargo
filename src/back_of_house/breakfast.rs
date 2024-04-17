pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: "peaches".to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        String::from(format!("Breakfast({} with {});", self.toast, self.seasonal_fruit))
    }
}
