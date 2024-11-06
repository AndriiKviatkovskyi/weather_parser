use pest::Parser;
use pest_derive::Parser;
use anyhow::anyhow;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct WeatherParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_condition_positive() {
        let input = "Condition: Clear";
        assert!(WeatherParser::parse(Rule::condition, input).is_ok());
    }

    #[test]
    fn test_condition_negative() {
        let input = "Condition: Weird";
        assert!(WeatherParser::parse(Rule::condition, input).is_err());
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
    fn test_wind_direction_positive() {
        let input = "Wind direction: NE";
        assert!(WeatherParser::parse(Rule::wind_direction, input).is_ok());
    }

    #[test]
    fn test_wind_direction_negative() {
        let input = "Wind direction: 5";
        assert!(WeatherParser::parse(Rule::wind_direction, input).is_err());
    }

    #[test]
    fn test_wind_speed_positive() {
        let input = "Wind speed: 12km/h";
        assert!(WeatherParser::parse(Rule::wind_speed, input).is_ok());
    }

    #[test]
    fn test_wind_speed_negative() {
        let input = "Wind speed: 12 mph";
        assert!(WeatherParser::parse(Rule::wind_speed, input).is_err());
    }

    #[test]
    fn test_precipitation_positive() {
        let input = "Precipitation type: Rain";
        assert!(WeatherParser::parse(Rule::precipitation, input).is_ok());
    }

    #[test]
    fn test_precipitation_negative() {
        let input = "Precipitation type: Frogs";
        assert!(WeatherParser::parse(Rule::precipitation, input).is_err());
    }

    #[test]
    fn test_precipitation_size_positive() {
        let input = "Precipitation size: 5.5mm";
        assert!(WeatherParser::parse(Rule::precipitation_size, input).is_ok());
    }

    #[test]
    fn test_precipitation_size_negative() {
        let input = "Precipitation size: 5m";
        assert!(WeatherParser::parse(Rule::precipitation_size, input).is_err());
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
    





