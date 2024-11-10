use anyhow::Result;
use pest::Parser;
use weather_parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_positive() -> Result<()> {
        let input = "30";
        assert!(WeatherParser::parse(Rule::integer, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_integer_negative() -> Result<()> {
        let input = "a";
        assert!(WeatherParser::parse(Rule::integer, input).is_err());
        Ok(())
    }

    #[test]
    fn test_float_positive() -> Result<()> {
        let input = "30.5";
        assert!(WeatherParser::parse(Rule::float, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_float_negative() -> Result<()> {
        let input = "a.b";
        assert!(WeatherParser::parse(Rule::float, input).is_err());
        Ok(())
    }

    #[test]
    fn test_time_positive() -> Result<()> {
        let input = "22:22";
        assert!(WeatherParser::parse(Rule::time, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_time_negative() -> Result<()> {
        let input = "aa:bb";
        assert!(WeatherParser::parse(Rule::time, input).is_err());
        Ok(())
    }

    #[test]
    fn test_string_positive() -> Result<()> {
        let input = "gugugaga";
        assert!(WeatherParser::parse(Rule::string, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_string_negative() -> Result<()> {
        let input = ";";
        assert!(WeatherParser::parse(Rule::string, input).is_err());
        Ok(())
    }

    #[test]
    fn test_string_collection_positive() -> Result<()> {
        let input = "gugugaga; biba";
        assert!(WeatherParser::parse(Rule::string_collection, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_string_collection_negative() -> Result<()> {
        let input = ";;;";
        assert!(WeatherParser::parse(Rule::string_collection, input).is_err());
        Ok(())
    }

    #[test]
    fn test_condition_positive() -> Result<()> {
        let input = "Condition: Clear";
        assert!(WeatherParser::parse(Rule::condition, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_condition_negative() -> Result<()> {
        let input = "Cond: Clear";
        assert!(WeatherParser::parse(Rule::condition, input).is_err());
        Ok(())
    }

    #[test]
    fn test_condition_type_positive() -> Result<()> {
        let input = "Clear";
        assert!(WeatherParser::parse(Rule::condition_type, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_condition_type_negative() -> Result<()> {
        let input = "Weird";
        assert!(WeatherParser::parse(Rule::condition_type, input).is_err());
        Ok(())
    }

    #[test]
    fn test_temperature_positive() -> Result<()> {
        let input = "Temperature: -23.5C";
        assert!(WeatherParser::parse(Rule::temperature, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_temperature_negative() -> Result<()> {
        let input = "Temperature: 23F";
        assert!(WeatherParser::parse(Rule::temperature, input).is_err());
        Ok(())
    }

    #[test]
    fn test_humidity_positive() -> Result<()> {
        let input = "Humidity: 50%";
        assert!(WeatherParser::parse(Rule::humidity, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_humidity_negative() -> Result<()> {
        let input = "Humidity: 50";
        assert!(WeatherParser::parse(Rule::humidity, input).is_err());
        Ok(())
    }

    #[test]
    fn test_wind_positive() -> Result<()> {
        let input = "Wind: NE 15km/h";
        assert!(WeatherParser::parse(Rule::wind, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_wind_negative() -> Result<()> {
        let input = "Wind: 15km/h NE";
        assert!(WeatherParser::parse(Rule::wind, input).is_err());
        Ok(())
    }

    #[test]
    fn test_wind_direction_positive() -> Result<()> {
        let input = "NE";
        assert!(WeatherParser::parse(Rule::direction, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_wind_direction_negative() -> Result<()> {
        let input = "east";
        assert!(WeatherParser::parse(Rule::direction, input).is_err());
        Ok(())
    }

    #[test]
    fn test_precipitation_positive() -> Result<()> {
        let input = "Precipitation: Rain 5mm";
        assert!(WeatherParser::parse(Rule::precipitation, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_precipitation_negative() -> Result<()> {
        let input = "Precipitation: Rain 5cm";
        assert!(WeatherParser::parse(Rule::precipitation, input).is_err());
        Ok(())
    }

    #[test]
    fn test_precipitation_type_positive() -> Result<()> {
        let input = "Rain";
        assert!(WeatherParser::parse(Rule::precipitation_type, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_precipitation_type_negative() -> Result<()> {
        let input = "Cold november rain";
        assert!(WeatherParser::parse(Rule::precipitation_type, input).is_err());
        Ok(())
    }

    #[test]
    fn test_visibility_positive() -> Result<()> {
        let input = "Visibility: 10km";
        assert!(WeatherParser::parse(Rule::visibility, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_visibility_negative() -> Result<()> {
        let input = "Visibility: far away";
        assert!(WeatherParser::parse(Rule::visibility, input).is_err());
        Ok(())
    }

    #[test]
    fn test_cloud_cover_positive() -> Result<()> {
        let input = "Cloud Cover: 75%";
        assert!(WeatherParser::parse(Rule::cloud_cover, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_cloud_cover_negative() -> Result<()> {
        let input = "Cloud Cover: 75";
        assert!(WeatherParser::parse(Rule::cloud_cover, input).is_err());
        Ok(())
    }

    #[test]
    fn test_cloud_types_positive() -> Result<()> {
        let input = "Cloud Types: Cirrus, Stratus";
        assert!(WeatherParser::parse(Rule::cloud_types, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_cloud_types_negative() -> Result<()> {
        let input = "Cloud Types: Nice Pretty";
        assert!(WeatherParser::parse(Rule::cloud_types, input).is_err());
        Ok(())
    }

    #[test]
    fn test_cloud_type_positive() -> Result<()> {
        let input = "Stratus";
        assert!(WeatherParser::parse(Rule::cloud_type, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_cloud_type_negative() -> Result<()> {
        let input = "Straus";
        assert!(WeatherParser::parse(Rule::cloud_type, input).is_err());
        Ok(())
    }

    #[test]
    fn test_pressure_positive() -> Result<()> {
        let input = "Pressure: 1013hPa";
        assert!(WeatherParser::parse(Rule::pressure, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_pressure_negative() -> Result<()> {
        let input = "Pressure: 1013Pa";
        assert!(WeatherParser::parse(Rule::pressure, input).is_err());
        Ok(())
    }

    #[test]
    fn test_uv_index_positive() -> Result<()> {
        let input = "UV Index: 5";
        assert!(WeatherParser::parse(Rule::uv_index, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_uv_index_negative() -> Result<()> {
        let input = "UV Index: five";
        assert!(WeatherParser::parse(Rule::uv_index, input).is_err());
        Ok(())
    }

    #[test]
    fn test_air_quality_positive() -> Result<()> {
        let input = "Air Quality: Good";
        assert!(WeatherParser::parse(Rule::air_quality, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_air_quality_negative() -> Result<()> {
        let input = "Air Quality: Excellent";
        assert!(WeatherParser::parse(Rule::air_quality, input).is_err());
        Ok(())
    }

    #[test]
    fn test_air_quality_level_positive() -> Result<()> {
        let input = "Good";
        assert!(WeatherParser::parse(Rule::air_quality_level, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_air_quality_level_negative() -> Result<()> {
        let input = "null";
        assert!(WeatherParser::parse(Rule::air_quality_level, input).is_err());
        Ok(())
    }

    #[test]
    fn test_sunrise_positive() -> Result<()> {
        let input = "Sunrise: 06:45";
        assert!(WeatherParser::parse(Rule::sunrise, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_sunrise_negative() -> Result<()> {
        let input = "Sunrise: 645";
        assert!(WeatherParser::parse(Rule::sunrise, input).is_err());
        Ok(())
    }

    #[test]
    fn test_sunset_positive() -> Result<()> {
        let input = "Sunset: 19:30";
        assert!(WeatherParser::parse(Rule::sunset, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_sunset_negative() -> Result<()> {
        let input = "Sunset: 7:30PM";
        assert!(WeatherParser::parse(Rule::sunset, input).is_err());
        Ok(())
    }
}
