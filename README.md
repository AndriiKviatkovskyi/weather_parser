## weather_parser

### Description
This project implements a parser that extracts and formats detailed weather condition data from structured text inputs. It can parse various weather parameters and format them into JSON.

### Technical description
The parser reads structured text inputs containing weather conditions, applies grammar rules and outputs the parsed results as structured data. It takes a string, which consists of following parts:

- **Condition**: Parses descriptive weather states like "Clear," "Partly Cloudy," "Thunderstorm," and more.
- **Temperature**: Extracts temperatures, supporting negative and positive values, and outputs them in Celsius (e.g., "Temperature: -5C").
- **Humidity**: Captures humidity levels as percentages (e.g., "Humidity: 85%").
- **Wind Direction**: Extracts wind direction and speed (e.g., "Wind: NW 13.4km/h").
- **Precipitation**: Identifies types of precipitation and its amount (e.g., "Precipitation: Rain 5mm").
- **Visibility**: Extracts visibility data measured in kilometers (e.g., "Visibility: 10.5km").
- **Cloud Cover**: Parses cloud cover percentages (e.g., "Cloud Cover: 60%").
- **Pressure**: Captures atmospheric pressure readings in hectopascals (e.g., "Pressure: 1013hPa").
- **UV Index**: Extracts UV index values for solar radiation exposure (e.g., "UV Index: 7").
- **Air Quality**: Interprets air quality levels using predefined descriptors like "Moderate" or "Hazardous."
- **Sunrise and Sunset**: Parses sunrise and sunset times in HH:MM format (e.g., "Sunrise: 06:15").
- **Cloud types**: Parses cloud types (e.g., "Cloud Types: Cirrus, Stratus").

After that, it splits it by ';' symbol using special grammar rule and analyzes, whether each part matches required grammar rule. With every match it is putting new object to HashMap, which is later transfered to JSON using serde_json and gets written into file, chosen by user in CLI.

### Applying
The parser allows user to convert intuitively understandable weather conditions to JSON format, required by many applications, which can save a lot of time and effort. It can be used for various applications, including:

- Sharing weather data.
- Data analysis for studying weather patterns over time, such as temperature changes or precipitation trends.
- Integration with visualization tools to present weather data in graphs and charts.
- Support for meteorological research.

The parser's output can be used to build historical weather reports, monitor real-time conditions, or assist in building predictive models for weather trends.