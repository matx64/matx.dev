---
title: "Device Discovery Adventures"
date: "Sep 02, 2025"
---

> 📌 TL;DR: If you're building device discovery, just use `mDNS`. It's battle-tested (Apple uses it) and more reliable compared to the other options.

Recently I decided to build a file synchronization app from scratch ([synche](https://github.com/matx64/synche)) and the first requirement was to allow devices to discover each other in my local network.

Since it was my first time implementing this feature, I did some research and found 3 popular solutions: `UDP Broadcast`, `UDP Multicast` and `mDNS`.

Initially, I chose UDP Broadcast because it met my requirements and seemed very simple to implement, however I ended up implementing all three options after running into problems during testing.

I'll explain each solution at a higher level and share my experience with their implementations.

## UDP Broadcast

UDP Broadcasting is a general purpose solution that _can_ be used for device discovery. A device sends a UDP packet to the network's broadcast address (e.g `192.168.1.255` or `255.255.255.255`). Every other device on the network receives this packet, as routers typically enable broadcast forwarding by default.

Yes, **every device in the network receives the packet**, it doesn't matter if it's actively "listening" on a UDP port or not.

With that in mind, my initial design for each peer looked like this:

1. Bind a UDP socket to `0.0.0.0:8888` and enable the socket's broadcast feature.
2. Periodically send a small _ping_ message to the broadcast address `255.255.255.255:8888`.
3. Concurrently receive and parse incoming socket messages:
   - If sender's address is new, mark device as discovered.
   - If sender is already known, update its `last_seen` timestamp.
4. Periodically expire devices whose `last_seen` is older than a configured TTL (timeout).
5. If a peer gracefully shuts down the app, send a _shutdown ping_ so other devices can immediately disconnect it.

This solution was very simple to implement with `tokio::net::UdpSocket` and worked fine initially. Unfortunately, after some testing, I started seeing strange behavior between my MacBook and Windows desktop: the desktop received the Mac's pings and handled them correctly, but the Mac never seemed to receive the desktop's messages.

I used `Wireshark` to monitor both computers and see whether the packets at least arrived at the network interface. The desktop's packets did not appear in the Mac's capture, so the problem was below the application layer as the packets never reached the Mac at all. The behavior was intermittent: sometimes discovery worked perfectly, other times it failed.

I tried every obvious option to fix:

- Restarted both machines and the router.
- Adjusted firewall configs, such as enabling the required ports or disabling the firewall completely.
- Double-checked application level implementation, including socket options, and tried different broadcast addresses and ports.
- Tested on both wired and Wi-Fi.
- Tested broadcasting with `netcat` instead of my app.

The intermittent issue persisted, so I decided to implement a new solution. I still don't know what caused it, maybe I missed something, or there was an issue with my hardware or drivers. If anyone has experienced this behavior and fixed it, please email me, I'm still curious. I'll make sure to update this article with the solution if it works properly.

## UDP Multicast

UDP Multicast is similar to UDP Broadcast, just with a key difference that makes it more efficient. As I mentioned, Broadcast sends the message to _all_ devices on the network, regardless of whether they requested it, while Multicast **only delivers it to devices that have explicitly joined a specific multicast group**.

In terms of implementation, the peer design was _identical_ to UDP Broadcast, the only difference was Tokio's `UdpSocket` options, which now required joining a multicast group (`239.255.0.1`).

Honestly, since both solutions are so fundamentally similar, I kinda expected the intermittent issue to persist, and it did, but in the opposite way. Now, my Desktop wasn't receiving the Mac's pings, while the Mac started working properly, all related to the network interface layer, not application.

I moved to my last resort: mDNS, which seemed more complex to implement but apparently it is the go-to option for device discovery nowadays.

## mDNS Service Discovery

When I was debugging the mentioned issue, I filtered `mdns` in Wireshark out of pure curiosity. A lot of captures appeared with, for example, my Airpod's metadata. If I'd only known that Apple relies on mDNS to power their seamless device discovery (AirPods, AirDrop), I would've started here. Given how well their ecosystem works in this regard, mDNS looked very promising from the start.

mDNS is a zero-configuration protocol that enables devices to discover services on a local network by using UDP Multicast packets under the hood. Shortly, **each device subscribes to the required services types and is able to register services with proper metadata**.

In practice, my per peer design became very different from the previous options:

1. Initialize a mDNS Service Daemon and start browsing for my app's service type: `_synche._udp.local.`.
2. Register its own service with metadata: service type, device id, hostname, local ip and port.
3. Concurrently process incoming events from the browsed service type:
   - If sender's address is new, mark device as discovered.
   - On a `Service Removed` event, disconnect peer.

The good thing about mDNS is that even if my app stops, abruptly or not, the Service Removed event will still be triggered. It happens because it's the network layer that triggers it, not the app. Even if the application explicitly calls the Daemon's `shutdown` method, this event isn't immediately triggered, it only triggers after a certain TTL (tipically 1-2 minutes) by the network layer.

This behavior encouraged me to add extra checks elsewhere in the app, like error handling for failed connections or retry logic during sync operations. These are best practices anyway, so it wasn't an issue, it actually improved overall reliability.

I've been testing the mDNS implementation for weeks now and **it is working flawlessly** across my Macbook and Windows Desktop, no more intermittent issues.

The only configuration I did was on MacOS, by executing these commands to ensure mDNS service was enabled (just in case, remember I was tilted with the previous issues):

```sh
# allow mDNS in MacOS
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --add /usr/libexec/mdnsd
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --unblock /usr/libexec/mdnsd
```

## Conclusion

Building device discovery for my file sync app was a very fun adventure! I started with the easier UDP Broadcast approach and ended up with a polished mDNS solution.

For those trying to implement the same functionality, I recommend preferring mDNS by default. Unless you have some specific requirement or limitation that wouldn't let you use it, based off my experience it turned out to be a more reliable solution.

If you want to see the actual code I used in Synche, here are the implementations I mentioned:

- [UDP Broadcast](https://github.com/matx64/synche/blob/e5bda6864c70255c1ab8962f1cb65596a2fd2ad5/src/infra/network/udp.rs)
- [UDP Multicast](https://github.com/matx64/synche/blob/26d15a8d6d701a5b81aa2180d7586344b3bb7ca8/src/infra/network/multicast.rs)
- [mDNS](https://github.com/matx64/synche/blob/5b068daafc57587260fae33cfe1afdd20682a53a/src/infra/network/mdns.rs)
