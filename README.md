# Return of Reckoning Launcher
_A simple command line launcher for Return of Reckoning_

### Features:
* Will check and download server information from the official Return of Reckoning website
* Will verify mythloginserviceconfig.xml and update it if necessary.
* Handles running via wine if running a unix family os (Mac/Linux)

### Usage:
1. Download and extract into your Warhammer Online directory
2. Run it:
    * Windows: `ror_launch.exe username password`
    * Unix: `./ror_launch username password`
### Building:
1. clone or download this repo
2. with a terminal / command prompt open the directory
3. cargo build --release 
4. cp target/release/ror_launch into your WAR directory
### Caveats
* This won't patch your data.myp so you'll have to grab an up to date one
* This won't hide your password from your terminal
* This won't look pretty