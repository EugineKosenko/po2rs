lazy_static::lazy_static! {
    pub static ref LANGUAGES: Vec<&'static str> = vec![
        {% for lang in langs %}
            "{{lang}}"{% if not loop.last %},{% endif %}
        {% endfor %}
    ];
    pub static ref TRANSLATIONS: Vec<(String, serde_json::Value)> = vec![
        {% for trans in trans %}
            ("{{trans.0}}".to_string(), serde_json::json!({
                {% for item in trans.1 %}
                    "{{item.0}}": "{{item.1}}"{% if not loop.last %},{% endif %}
                {% endfor %}
            })){% if not loop.last %},{% endif %}
        {% endfor %}
    ];
}
