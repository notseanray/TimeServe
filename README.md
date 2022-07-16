#### TimeServe

simple time sync tool, I installed open-ntp and it was simply unnecessary. I dual boot windows + gnu/linux and do not need a daemon running to sync the time. I wrote this tiny tool to have a small program run on a server while a client can connect and convert the server's unix timestamp into the local time/date and set it with root permissions. I should have probably used something like Python for this since it wouldn't need this extra fat binary.

Creates an HTTP endpoint to request the UNIX timestamp in ms. You can run the server portion with ./TimeServe --serve, any other argument will be seen as a server ip to connect to.


##### Examples:
```
// server
$ ./TimeServe --serve 54321
// sync
# ./TimeServe 192.168.1.0:54321
```
