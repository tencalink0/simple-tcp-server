# simple-tcp-server
A basic tcp server which returns html templates dynamically (from /public)<br>
This was build solely as a proof-of-concept but also is being used for [b_glossa](https://github.com/bev29rr/b_glossa)

## How to setup
1. Download all server files into desired folders
2. Connect crate to project
3. Initialise a new server ```Server::from_presets()``` or ```Server::new(ip, port)```
4. Run the server ```web_server.start()```

## Version 1.0 <sub><sup>(c2118b147ee35c9df6ca26f1cbf43e3f074030b8)</sup></sub>
### Key Features:
- Dynamically accessing files from index
- Dynamically accessing templates from the required folders
- Properly structured objects
- ANSII colouring to display server side connections
