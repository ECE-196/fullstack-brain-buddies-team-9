
| Name | Description | Default value |
|------|-------------|---------------|
|**ESP_WIFI_RX_QUEUE_SIZE**|Size of the RX queue in frames|5|
|**ESP_WIFI_TX_QUEUE_SIZE**|Size of the TX queue in frames|3|
|**ESP_WIFI_STATIC_RX_BUF_NUM**|WiFi static RX buffer number. See [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/network/esp_wifi.html#_CPPv418wifi_init_config_t)|10|
|**ESP_WIFI_DYNAMIC_RX_BUF_NUM**|WiFi dynamic RX buffer number. See [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/network/esp_wifi.html#_CPPv418wifi_init_config_t)|32|
|**ESP_WIFI_STATIC_TX_BUF_NUM**|WiFi static TX buffer number. See [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/network/esp_wifi.html#_CPPv418wifi_init_config_t)|0|
|**ESP_WIFI_DYNAMIC_TX_BUF_NUM**|WiFi dynamic TX buffer number. See [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/network/esp_wifi.html#_CPPv418wifi_init_config_t)|32|
|**ESP_WIFI_AMPDU_RX_ENABLE**|WiFi AMPDU RX feature enable flag. See [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/network/esp_wifi.html#_CPPv418wifi_init_config_t)|false|
|**ESP_WIFI_AMPDU_TX_ENABLE**|WiFi AMPDU TX feature enable flag. See [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/network/esp_wifi.html#_CPPv418wifi_init_config_t)|false|
|**ESP_WIFI_AMSDU_TX_ENABLE**|WiFi AMSDU TX feature enable flag. See [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/network/esp_wifi.html#_CPPv418wifi_init_config_t)|false|
|**ESP_WIFI_RX_BA_WIN**|WiFi Block Ack RX window size. See [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/network/esp_wifi.html#_CPPv418wifi_init_config_t)|6|
|**ESP_WIFI_MAX_BURST_SIZE**|See [smoltcp's documentation](https://docs.rs/smoltcp/0.10.0/smoltcp/phy/struct.DeviceCapabilities.html#structfield.max_burst_size)|1|
|**ESP_WIFI_COUNTRY_CODE**|Country code. See [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-guides/wifi.html#wi-fi-country-code)|CN|
|**ESP_WIFI_COUNTRY_CODE_OPERATING_CLASS**|If not 0: Operating Class table number. See [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-guides/wifi.html#wi-fi-country-code)|0|
|**ESP_WIFI_MTU**|MTU, see [smoltcp's documentation](https://docs.rs/smoltcp/0.10.0/smoltcp/phy/struct.DeviceCapabilities.html#structfield.max_transmission_unit)|1492|
|**ESP_WIFI_TICK_RATE_HZ**|Tick rate of the internal task scheduler in hertz|100|
|**ESP_WIFI_LISTEN_INTERVAL**|Interval for station to listen to beacon from AP. The unit of listen interval is one beacon interval. For example, if beacon interval is 100 ms and listen interval is 3, the interval for station to listen to beacon is 300 ms|3|
|**ESP_WIFI_BEACON_TIMEOUT**|For Station, If the station does not receive a beacon frame from the connected SoftAP during the  inactive time, disconnect from SoftAP. Default 6s. Range 6-30|6|
|**ESP_WIFI_AP_BEACON_TIMEOUT**|For SoftAP, If the SoftAP doesn’t receive any data from the connected STA during inactive time, the SoftAP will force deauth the STA. Default is 300s|300|
|**ESP_WIFI_FAILURE_RETRY_CNT**|Number of connection retries station will do before moving to next AP. scan_method should be set as WIFI_ALL_CHANNEL_SCAN to use this config. Note: Enabling this may cause connection time to increase incase best AP doesn't behave properly. Defaults to 1|1|
|**ESP_WIFI_SCAN_METHOD**|0 = WIFI_FAST_SCAN, 1 = WIFI_ALL_CHANNEL_SCAN, defaults to 0|0|
