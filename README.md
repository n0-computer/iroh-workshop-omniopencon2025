# Iroh and iroh-blobs workshop for Web3 Summit 2025

[Slides](https://docs.google.com/presentation/d/e/2PACX-1vR5anKQBBwN69DDBcxKxO055Td39Dg_y5DZeQSgzcWRlmLOzbofGU_ThnteONllmO80gdlPoowLnhLi/pub?start=false&loop=false&delayms=3000)

[Workshop walkthrough](https://hackmd.io/RkVPciqHTH2eekd29pQYyQ?view)

[Iroh doctor](https://crates.io/crates/iroh-doctor)

iroh-doctor can be used to check connection status and performance. Install with `cargo install iroh-doctor`.

# Chat

<iroh.computer/discord> channel `#workshop`

# Wifi

You can use any wifi. You will *always* be able to establish a connection as long as you are connected to the internet in some way.

Here is a phone hotspot in case conference wifi does not work:

SSID: `iroh`
Pass: `movethebytes`

# Exercises

The workshop is structured into multiple exercises. Each exercise is a
self-contained example and can be run using cargon run -p <lessonname>.

There is a branch `diy` where the important code is removed, and you can try
to code the exercises. But is also fine to just use `main` and follow along.

## Echo 1

A simple echo service, to show the basics of iroh connections

```
cargo run -p echo1
```

## Echo 2

Echo service from before, but done as a iroh protocol handler

```
cargo run -p echo2
```

## Echo 3

Echo service from before, but show intricacies of node discovery

```
cargo run -p echo3
```

## Sendme 1

Uses iroh-blobs to send a single file, done as an iroh protocol handler

```
cargo run -p sendme1
```

## Sendme 2

Uses iroh-blobs to send a directory, done as a protocol handler

```
cargo run -p sendme2
```

## Sendme 3

Introducing the downloader. Receive from multiple senders at the same time.

```
cargo run -p sendme3
```

## Sendme 4

Publish to a content discovery service, and use that service to find providers
for the content on the receive side

```
cargo run -p sendme4
```

# Notes

<b>Note: the workshop is using an *alpha* version of iroh-blobs.</b>

The stable version of iroh and iroh-blobs is 0.35. The 0.9x series is changing
rapidly as we work towards a 1.0 release.

Docs are at https://docs.rs/iroh-blobs/latest/iroh_blobs/

The stable version of iroh-blobs on crates.io has most of the same features, but
the API has been restructured in preparation of releasing iroh-blobs 1.0.

If you want to see an implementation of blake3 data transfer that has proper
progress bars, check out https://github.com/n0-computer/sendme