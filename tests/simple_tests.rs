use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct WeatherParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_positive() {
        let input = "30";
        assert!(WeatherParser::parse(Rule::integer, input).is_ok());
    }

    #[test]
    fn test_integer_negative() {
        let input = "a";
        assert!(WeatherParser::parse(Rule::integer, input).is_err());
    }

    #[test]
    fn test_float_positive() {
        let input = "30.5";
        assert!(WeatherParser::parse(Rule::float, input).is_ok());
    }

    #[test]
    fn test_float_negative() {
        let input = "a.b";
        assert!(WeatherParser::parse(Rule::float, input).is_err());
    }

    #[test]
    fn test_time_positive() {
        let input = "22:22";
        assert!(WeatherParser::parse(Rule::time, input).is_ok());
    }

    #[test]
    fn test_time_negative() {
        let input = "aa:bb";
        assert!(WeatherParser::parse(Rule::time, input).is_err());
    }

    #[test]
    fn test_string_positive() {
        let input = "gugugaga";
        assert!(WeatherParser::parse(Rule::string, input).is_ok());
    }

    #[test]
    fn test_string_negative() {
        let input = ";";
        assert!(WeatherParser::parse(Rule::string, input).is_err());
    }

    #[test]
    fn test_string_collection_positive() {
        let input = "gugugaga; biba";
        assert!(WeatherParser::parse(Rule::string_collection, input).is_ok());
    }

    #[test]
    fn test_string_collection_negative() {
        let input = ";;;";
        assert!(WeatherParser::parse(Rule::string_collection, input).is_err());
    }

    #[test]
    fn test_condition_positive() {
        let input = "Condition: Clear";
        assert!(WeatherParser::parse(Rule::condition, input).is_ok());
    }

    #[test]
    fn test_condition_negative() {
        let input = "Cond: Clear";
        assert!(WeatherParser::parse(Rule::condition, input).is_err());
    }

    #[test]
    fn test_condition_type_positive() {
        let input = "Clear";
        assert!(WeatherParser::parse(Rule::condition_type, input).is_ok());
    }

    #[test]
    fn test_condition_type_negative() {
        let input = "Weird";
        assert!(WeatherParser::parse(Rule::condition_type, input).is_err());
    }


    #[test]
    fn test_temperature_positive() {
        let input = "Temperature: -23.5C";
        assert!(WeatherParser::parse(Rule::temperature, input).is_ok());
    }

    #[test]
    fn test_temperature_negative() {
        let input = "Temperature: 23F";
        assert!(WeatherParser::parse(Rule::temperature, input).is_err());
    }

    #[test]
    fn test_humidity_positive() {
        let input = "Humidity: 50%";
        assert!(WeatherParser::parse(Rule::humidity, input).is_ok());
    }

    #[test]
    fn test_humidity_negative() {
        let input = "Humidity: 50";
        assert!(WeatherParser::parse(Rule::humidity, input).is_err());
    }

    #[test]
    fn test_wind_positive() {
        let input = "Wind: NE 15km/h";
        assert!(WeatherParser::parse(Rule::wind, input).is_ok());
    }

    #[test]
    fn test_wind_negative() {
        let input = "Wind: 15km/h NE";
        assert!(WeatherParser::parse(Rule::wind, input).is_err());
    }

    #[test]
    fn test_wind_direction_positive() {
        let input = "NE";
        assert!(WeatherParser::parse(Rule::direction, input).is_ok());
    }

    #[test]
    fn test_wind_direction_negative() {
        let input = "east";
        assert!(WeatherParser::parse(Rule::direction, input).is_err());
    }

    #[test]
    fn test_precipitation_positive() {
        let input = "Precipitation: Rain 5mm";
        assert!(WeatherParser::parse(Rule::precipitation, input).is_ok());
    }

    #[test]
    fn test_precipitation_negative() {
        let input = "Precipitation: Rain 5cm";
        assert!(WeatherParser::parse(Rule::precipitation, input).is_err());
    }

    #[test]
    fn test_precipitation_type_positive() {
        let input = "Rain";
        assert!(WeatherParser::parse(Rule::precipitation_type, input).is_ok());
    }

    #[test]
    fn test_precipitation_type_negative() {
        let input = "Cold november rain";
        assert!(WeatherParser::parse(Rule::precipitation_type, input).is_err());
    }

    #[test]
    fn test_visibility_positive() {
        let input = "Visibility: 10km";
        assert!(WeatherParser::parse(Rule::visibility, input).is_ok());
    }

    #[test]
    fn test_visibility_negative() {
        let input = "Visibility: far away";
        assert!(WeatherParser::parse(Rule::visibility, input).is_err());
    }

    #[test]
    fn test_cloud_cover_positive() {
        let input = "Cloud Cover: 75%";
        assert!(WeatherParser::parse(Rule::cloud_cover, input).is_ok());
    }

    #[test]
    fn test_cloud_cover_negative() {
        let input = "Cloud Cover: 75";
        assert!(WeatherParser::parse(Rule::cloud_cover, input).is_err());
    }

    #[test]
    fn test_cloud_types_positive() {
        let input = "Cloud Types: Cirrus, Stratus";
        assert!(WeatherParser::parse(Rule::cloud_types, input).is_ok());
    }

    #[test]
    fn test_cloud_types_negative() {
        let input = "Cloud Types: Nice Pretty";
        assert!(WeatherParser::parse(Rule::cloud_types, input).is_err());
    }

    #[test]
    fn test_cloud_type_positive() {
        let input = "Stratus";
        assert!(WeatherParser::parse(Rule::cloud_type, input).is_ok());
    }

    #[test]
    fn test_cloud_type_negative() {
        let input = "Straus";
        assert!(WeatherParser::parse(Rule::cloud_type, input).is_err());
    }

    #[test]
    fn test_pressure_positive() {
        let input = "Pressure: 1013hPa";
        assert!(WeatherParser::parse(Rule::pressure, input).is_ok());
    }

    #[test]
    fn test_pressure_negative() {
        let input = "Pressure: 1013Pa";
        assert!(WeatherParser::parse(Rule::pressure, input).is_err());
    }

    #[test]
    fn test_uv_index_positive() {
        let input = "UV Index: 5";
        assert!(WeatherParser::parse(Rule::uv_index, input).is_ok());
    }

    #[test]
    fn test_uv_index_negative() {
        let input = "UV Index: five";
        assert!(WeatherParser::parse(Rule::uv_index, input).is_err());
    }

    #[test]
    fn test_air_quality_positive() {
        let input = "Air Quality: Good";
        assert!(WeatherParser::parse(Rule::air_quality, input).is_ok());
    }

    #[test]
    fn test_air_quality_negative() {
        let input = "Air Quality: Excellent";
        assert!(WeatherParser::parse(Rule::air_quality, input).is_err());
    }

    #[test]
    fn test_air_quality_level_positive() {
        let input = "Good";
        assert!(WeatherParser::parse(Rule::air_quality_level, input).is_ok());
    }

    #[test]
    fn test_air_quality_level_negative() {
        let input = "null";
        assert!(WeatherParser::parse(Rule::air_quality_level, input).is_err());
    }


    #[test]
    fn test_sunrise_positive() {
        let input = "Sunrise: 06:45";
        assert!(WeatherParser::parse(Rule::sunrise, input).is_ok());
    }

    #[test]
    fn test_sunrise_negative() {
        let input = "Sunrise: 645";
        assert!(WeatherParser::parse(Rule::sunrise, input).is_err());
    }

    #[test]
    fn test_sunset_positive() {
        let input = "Sunset: 19:30";
        assert!(WeatherParser::parse(Rule::sunset, input).is_ok());
    }

    #[test]
    fn test_sunset_negative() {
        let input = "Sunset: 7:30PM";
        assert!(WeatherParser::parse(Rule::sunset, input).is_err());
    }

}
    





