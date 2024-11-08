use pest::Parser;
use pest_derive::Parser;
use serde_json::json;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct WeatherParser;

pub fn parse_input(input: &str) -> Result<HashMap<String, serde_json::Value>, String> {
    let mut weather_data = HashMap::new();

    if let Ok(mut parsed) = WeatherParser::parse(Rule::string_collection, input) {
        if let Some(collection) = parsed.next() {
            let strings: Vec<_> = collection.into_inner().map(|pair| pair.as_str().trim()).collect();
            let rules = [
                Rule::condition,
                Rule::temperature,
                Rule::humidity,
                Rule::wind,
                Rule::precipitation,
                Rule::visibility,
                Rule::cloud_cover,
                Rule::pressure,
                Rule::uv_index,
                Rule::air_quality,
                Rule::sunrise,
                Rule::sunset,
                Rule::cloud_types,
            ];

            for (i, string) in strings.iter().enumerate() {
                if i < rules.len() {
                    if let Ok(mut parsed) = WeatherParser::parse(rules[i], string) {
                        if let Some(record) = parsed.next() {
                            match rules[i] {
                                Rule::wind => {
                                    let mut wind_data = HashMap::new();
                                    for inner in record.into_inner() {
                                        match inner.as_rule() {
                                            Rule::direction => {
                                                wind_data.insert("Direction", inner.as_str().trim().to_string());
                                            }
                                            Rule::float | Rule::integer => {
                                                wind_data.insert("Speed", inner.as_str().trim().to_string());
                                            }
                                            _ => {}
                                        }
                                    }
                                    weather_data.insert("Wind".to_string(), json!(wind_data));
                                }
                                Rule::precipitation => {
                                    let mut precipitation_data = HashMap::new();
                                    for inner in record.into_inner() {
                                        match inner.as_rule() {
                                            Rule::precipitation_type => {
                                                precipitation_data.insert("Type", inner.as_str().trim().to_string());
                                            }
                                            Rule::float | Rule::integer => {
                                                precipitation_data.insert("Amount", inner.as_str().trim().to_string());
                                            }
                                            _ => {}
                                        }
                                    }
                                    weather_data.insert("Precipitation".to_string(), json!(precipitation_data));
                                }
                                Rule::cloud_types => {
                                    let cloud_types: Vec<String> = record.into_inner()
                                        .map(|inner| inner.as_str().trim().to_string())
                                        .collect();
                                    weather_data.insert("Cloud Types".to_string(), json!(cloud_types));
                                }
                                _ => {
                                    let key = match rules[i] {
                                        Rule::condition => "Condition",
                                        Rule::temperature => "Temperature",
                                        Rule::humidity => "Humidity",
                                        Rule::visibility => "Visibility",
                                        Rule::cloud_cover => "Cloud Cover",
                                        Rule::pressure => "Pressure",
                                        Rule::uv_index => "UV Index",
                                        Rule::air_quality => "Air Quality",
                                        Rule::sunrise => "Sunrise",
                                        Rule::sunset => "Sunset",
                                        Rule::cloud_types => "Cloud Types",
                                        _ => "Unknown",
                                    };
                                    weather_data.insert(key.to_string(), json!(record.as_str().trim().to_string()));
                                }
                            }
                        } else {
                            return Err(format!("String '{}' did not match rule {:?} (Rule: {:?})", string, rules[i], rules[i]));
                        }
                    } else {
                        return Err(format!("String '{}' did not match rule {:?} (Rule: {:?})", string, rules[i], rules[i]));
                    }
                }
            }
        }
    }

    if weather_data.is_empty() {
        Err("No matching rule found".to_string())
    } else {
        Ok(weather_data)
    }
}