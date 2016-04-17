# SerialServoBridgeServer [![Build Status](https://travis-ci.org/Nixes/SerialServoBridgeServer.svg)](https://travis-ci.org/Nixes/SerialServoBridgeServer)
TCP->Serial Bridge, for controlling servos
Connects to an arduino servo controller over a serial connection and
allows control using a tcp connection.

Arduino sketch supplied in root folder

Currently only controls two servos, but should be easy to add more.

#Protocol
0xFE, pan (as an integer), tit (as an integer), 0xFF
