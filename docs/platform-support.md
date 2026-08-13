# Platform Support

Linux is the primary platform for `0.5.0`. Its backend reads stable local
interfaces and works independently of the init system.

The BSD backends are separated by target so platform-specific system APIs do
not leak into the Linux implementation. FreeBSD, OpenBSD, NetBSD and DragonFly
BSD are release targets. FreeBSD has a VM CI job; the other BSD targets remain
pre-release until native CI jobs and smoke tests are available.

Supported data is allowed to vary by platform. A field that is unavailable on
a particular kernel should display `Unknown`, not fail the whole fetch.
