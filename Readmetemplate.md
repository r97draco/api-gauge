# API-GAUGE
[![Open Source Love](https://badges.frapsoft.com/os/v1/open-source.png?v=103)](https://github.com/ellerbrock/open-source-badges/)
[![MIT Licence](https://badges.frapsoft.com/os/mit/mit.png?v=103)](https://opensource.org/licenses/mit-license.php)
<br>
*Built with Rust*
<br> 
<h2>Functionality :</h2> Measure how your API performs when a lot of concurrent requests are made.

**Webpage:** [lsusb-ui](https://singh47.github.io/api-gauge/)

## How to start
1. ```git clone https://github.com/singh47/api-gauge.git ```
2. Run following command to run the code (sends 20 requests by default)
<br>```cargo run```
2. Run following in terminal
<br>```npm run [number_of_requests]```

## Showcase
![alt text](mac-ss.png)

## Dependencies
- reqwest = { version = "0.11.14", features = ["json"] }
- tokio = { version = "1.24.2", features = ["full"] }
- serde = { version = "1.0.152", features = ["derive"] }
- serde_json = "1.0.91"
- futures = "0.3.25"


<!-- **Members:**
1. [Jagroop Singh](https://github.com/r97draco)
2. [Harmandeep Singh](https://github.com/singh47) -->