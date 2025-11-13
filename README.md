# Vein Dashboard

This is native, very light web server for Vein game.
It works in background consuming almost no resources.

Once compiled it can run on any Linux and will use Server Side Events to send updates about the game.

## Config file

You can change 

```toml
# Web server config
bind = "0.0.0.0"
port = 8765

# Vein dedicated server config
vein_server_bind = "http://vein.ita-prog.pl"
vein_server_port = 8080

# Interval how often emit events
refresh_interval_sec = 5
```

## Screenshots

![./assets/status.png](./assets/status.png)
![./assets/logs.png](./assets/logs.png)

## Roudmap

[ ] Install command
[ ] Icons
[ ] Web config manipulation
[ ] Vein dedicated server managment

