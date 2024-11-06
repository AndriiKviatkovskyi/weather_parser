### Weather Conditions Parser

### Description
This project implements a parser that extracts and formats detailed weather condition data from structured text inputs. It can parse various weather parameters and format them into a structured format for analysis.

### Technical description
The parser reads structured text inputs containing weather conditions, applies grammar rules and outputs the parsed results as structured data.

- **Condition**: Parses descriptive weather states like "Clear," "Partly Cloudy," "Thunderstorm," and more.
- **Temperature**: Extracts temperatures, supporting negative and positive values, and outputs them in Celsius (e.g., "Temperature: -5C").
- **Humidity**: Captures humidity levels as percentages (e.g., "Humidity: 85%").
- **Wind Direction**: Extracts wind direction (e.g., "Wind direction: NW").
- **Wind Speed**: Parses wind speed, supporting both integer and float values (e.g., "Wind speed: 15.5km/h").
- **Precipitation Type**: Identifies types of precipitation (e.g., "Precipitation type: Rain").
- **Precipitation Size**: Extracts the amount of precipitation in millimeters (e.g., "Precipitation size: 2.3mm").
- **Visibility**: Extracts visibility data measured in kilometers (e.g., "Visibility: 10.5km").
- **Cloud Cover**: Parses cloud cover percentages (e.g., "Cloud Cover: 60%").
- **Pressure**: Captures atmospheric pressure readings in hectopascals (e.g., "Pressure: 1013hPa").
- **UV Index**: Extracts UV index values for solar radiation exposure (e.g., "UV Index: 7").
- **Air Quality**: Interprets air quality levels using predefined descriptors like "Moderate" or "Hazardous."
- **Sunrise and Sunset**: Parses sunrise and sunset times in HH:MM format (e.g., "Sunrise: 06:15").

### Applying
The parser can be used for various applications, including:

- Weather forecasting systems that need to parse and format input data from different sources.
- Data analysis for studying weather patterns over time, such as temperature changes or precipitation trends.
- Integration with visualization tools to present weather data in graphs and charts.
- Support for meteorological research by converting raw weather condition data into analyzable formats.

The parser's output can be used to build historical weather reports, monitor real-time conditions, or assist in building predictive models for weather trends.