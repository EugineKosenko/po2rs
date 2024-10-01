lazy_static::lazy_static! {
    pub static ref LANGUAGES: Vec<&'static str> = vec![
        
            "en",
        
            "pl",
        
            "uk"
        
    ];
    pub static ref TRANSLATIONS: Vec<(String, serde_json::Value)> = vec![
        
            ("en".to_string(), serde_json::json!({
                
                    "Error": "Error",
                
                    "System Error": "System Error",
                
                    "Unexpected error": "Unexpected error",
                
                    "HTTP Error": "HTTP Error",
                
                    "E-mail": "E-mail",
                
                    "Password": "Password",
                
                    "Sign In": "Sign In",
                
                    "Sign Out": "Sign Out"
                
            })),
        
            ("pl".to_string(), serde_json::json!({
                
                    "Error": "Błąd",
                
                    "System Error": "Błąd systemu",
                
                    "Unexpected error": "Niespodziewany błąd",
                
                    "HTTP Error": "Błąd HTTP",
                
                    "E-mail": "E-mail",
                
                    "Password": "Hasło",
                
                    "Sign In": "Zalogować Się",
                
                    "Sign Out": "Wyloguj Się"
                
            })),
        
            ("uk".to_string(), serde_json::json!({
                
                    "Error": "Помилка",
                
                    "System Error": "Помилка системи",
                
                    "Unexpected error": "Неочікувана помилка",
                
                    "HTTP Error": "Помилка HTTP",
                
                    "E-mail": "E-mail",
                
                    "Password": "Пароль",
                
                    "Sign In": "Увійти",
                
                    "Sign Out": "Вийти"
                
            }))
        
    ];
}
