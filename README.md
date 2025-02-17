
[//]: # (Copyright 2019-2020 Rodolphe Bréard <rodolphe@breard.tf>)

[//]: # (Copying and distribution of this file, with or without modification,)
[//]: # (are permitted in any medium without royalty provided the copyright)
[//]: # (notice and this notice are preserved.  This file is offered as-is,)
[//]: # (without any warranty.)

# ACMEd

[![Build Status](https://github.com/breard-r/acmed/actions/workflows/ci.yml/badge.svg)](https://github.com/breard-r/acmed/actions/workflows/ci.yml)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.46.0+-lightgray.svg)](#build-from-source)
[![LICENSE MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT.txt)
[![LICENSE Apache 2.0](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE-APACHE-2.0.txt)

The Automatic Certificate Management Environment (ACME), is an internet standard ([RFC 8555](https://tools.ietf.org/html/rfc8555)) which allows to automate X.509 certificates signing by a Certification Authority (CA). ACMEd is one of the many clients for this protocol.


## Key features

- http-01, dns-01 and [tls-alpn-01](https://tools.ietf.org/html/rfc8737) challenges
- IP identifier validation extension [RFC 8738](https://tools.ietf.org/html/rfc8738)
- RSA 2048, RSA 4096, ECDSA P-256, ECDSA P-384, ECDSA P-521, Ed25519 and Ed448 certificates and account keys
- Internationalized domain names support
- Fully customizable challenge validation action
- Fully customizable archiving method (yes, you can use git or anything else)
- Nice and simple configuration file
- A pre-built set of hooks that can be used in most circumstances
- Run as a deamon: no need to set-up timers, crontab or other time-triggered process
- Retry of HTTPS request rejected with a badNonce or other recoverable errors
- Customizable HTTPS requests rate limits
- External account binding
- Optional key pair reuse (useful for [HPKP](https://en.wikipedia.org/wiki/HTTP_Public_Key_Pinning))
- For a given certificate, each domain name may be validated using a different challenge
- A standalone server dedicated to the tls-alpn-01 challenge validation (tacd)


## Planned features

- STAR certificates [RFC 8739](https://tools.ietf.org/html/rfc8739)
- Daemon and certificates management via the `acmectl` tool
- HTTP/2 support


## Project status

This project is usable, but is still a work in progress. Each release should works well and accordingly to its documentation.
Because the API has not been stabilized yet, breaking changes may occur. Therefore, before any upgrade, you are invited to read the [CHANGELOG](CHANGELOG.md) and check if any change can break your setup.

Please keep in mind this software has neither been subject to a peer review nor to a security audit.


## Documentation

The [wiki](https://github.com/breard-r/acmed/wiki) will provides you with an overview as well as guides. You may contribute to it by creating a PR on the [acmed-wiki](https://github.com/breard-r/acmed-wiki) repository.

For exhaustive references, the following man pages are available:

- acmed (8)
- acmed.toml (5)
- tacd (8)

An easy way to read those pages without installing ACMEd is to downloads and pipe them to the man utility:

```
curl -sSf "https://raw.githubusercontent.com/breard-r/acmed/main/man/en/acmed.8" | man -l -
curl -sSf "https://raw.githubusercontent.com/breard-r/acmed/main/man/en/acmed.toml.5" | man -l -
curl -sSf "https://raw.githubusercontent.com/breard-r/acmed/main/man/en/tacd.8" | man -l -
```

Alternatively, using zsh, you can use the following variants. Useful on system where man is unable to read from stdin (yes BSD, that's you).

```
man =(curl -sSf "https://raw.githubusercontent.com/breard-r/acmed/main/man/en/acmed.8")
man =(curl -sSf "https://raw.githubusercontent.com/breard-r/acmed/main/man/en/acmed.toml.5")
man =(curl -sSf "https://raw.githubusercontent.com/breard-r/acmed/main/man/en/tacd.8")
```


## Build from source

In order to compile ACMEd, you will need the [Rust](https://www.rust-lang.org/) compiler and its package manager, Cargo. The minimal required Rust version is 1.54, although it is recommended to use the latest stable one.

ACMEd depends OpenSSL 1.1.0 or higher.

On systems based on Debian/Ubuntu, you may need to install the `libssl-dev`, `build-essential` and `pkg-config` packages.

On Alpine Linux, you may need to install the `openssl-dev` and `alpine-sdk` packages.

```
$ make
$ make install
```

To build ACMEd and tacd inside a temporary Docker container, use the `contrib/docker/build-docker.sh` helper script. It currently supports Debian Buster / Stretch.

### Advanced options

You can specify a space or comma separated list of features to activate in the `FEATURE` variable. The possible features are:

- `openssl_dyn` (default): use OpenSSL as the cryptographic library, dynamically linked (mutually exclusive with `openssl_vendored`).
- `openssl_vendored`: use OpenSSL as the cryptographic library, statically linked (mutually exclusive with `openssl_dyn`).

You can also specify the [target triple](https://doc.rust-lang.org/nightly/rustc/platform-support.html) to build for in the `TARGET` variable. Please note that, if used, this variable must be specified for both `make` and `make install`.

For example, you can build statically linked binaries using the `openssl_vendored` feature and the `x86_64-unknown-linux-musl` target.

```
make FEATURES="openssl_vendored" TARGET="x86_64-unknown-linux-musl"
```

The following environment variables can be used to change default values at compile and/or install time:

- `PREFIX` (install): system user prefix (default to `/usr`)
- `BINDIR` (install): system binary directory (default to `$PREFIX/bin`)
- `DATADIR` (install): system data directory (default to `$PREFIX/share`)
- `MAN5DIR` (install): system directory where pages 5 manuals are located (default to `$DATADIR/man/man5`)
- `MAN8DIR` (install): system directory where pages 8 manuals are located (default to `$DATADIR/man/man8`)
- `SYSCONFDIR` (compile and install): system configuration directory (default to `/etc`)
- `VARLIBDIR` (compile and install): directory for persistent data modified by ACMEd (default to `/var/lib`)
- `RUNSTATEDIR` (compile): system run-time variable data (default to `/run`)
- `ACMED_DEFAULT_ACCOUNTS_DIR` (compile): directory where account files are stored (default to `$VARLIBDIR/acmed/accounts`)
- `ACMED_DEFAULT_CERT_DIR` (compile): directory where certificates and private keys are stored (default to `$VARLIBDIR/acmed/certs`)
- `ACMED_DEFAULT_CERT_FORMAT` (compile): format for certificates and private keys files names (default to `{ name }_{ key_type }.{ file_type }.{ ext }`)
- `ACMED_DEFAULT_CONFIG_FILE` (compile): main configuration file (default to `$SYSCONFDIR/acmed/acmed.toml`)
- `ACMED_DEFAULT_PID_FILE` (compile): PID file for the main acmed process (default to `$RUNSTATEDIR/acmed.pid`)
- `TACD_DEFAULT_PID_FILE` (compile): PID file for the tacd process (default to `$RUNSTATEDIR/tacd.pid`)

For example, the following will compile a binary that will use the `/usr/share/etc/acmed/acmed.toml` configuration file and will be installed in the `/usr/local/bin` directory :

```
make SYSCONFDIR="/usr/share/etc"
make BINDIR="/usr/local/bin" install
```

### Packaging

Most of the time, when packaging, you want to install the program in a dedicated directory. This is possible using the `DESTDIR` variable.

```
make DESTDIR="/path/to/my/package/directory" install
```

Packager tip: If you package ACMEd in a way it does not run as root, you might want to create another package that provides the Polkit rule file located in the `contrib/polkit` directory. This package should depends on both acmed and Polkit.


## Frequently Asked Questions

### Why this project?

After testing multiple ACME clients, I found out none of them supported all the features I expected (see the key features above). It may have been possible to contribute or fork an existing project, however I believe those project made architectural choices incompatible with what i wanted, and therefore it would be as much or less work to start a new project from scratch.

### Is it free and open-source software?

Yes, ACMEd is dual-licensed under the MIT and Apache 2.0 terms. See [LICENSE-MIT.txt](LICENSE-MIT.txt) and [LICENSE-APACHE-2.0.txt](LICENSE-APACHE-2.0.txt) for details.

The man pages, the default hooks configuration file, the `CHANGELOG.md` and the `README.md` files are released under the [GNU All-Permissive License](https://www.gnu.org/prep/maintain/html_node/License-Notices-for-Other-Files.html).

### Can it automatically change my server configuration?

Short answer: No.

Long answer: At some points in a certificate's life, ACMEd triggers some hooks in order to let you customize how some actions are done, therefore you can use those hooks to modify any server configuration you wish. However, this may not be what you are looking for since it cannot proactively detect which certificates should be emitted since ACMEd only manages certificates that have already been declared in the configuration files.

### How should I configure my TLS server?

You decide. ACMEd only retrieve the certificate for you, it does not impose any specific configuration or limitation on how to use it. For the record, if you are looking for security recommendations on TLS deployment, you can follow the [ANSSI TLS guide](https://www.ssi.gouv.fr/en/guide/security-recommendations-for-tls/) (the english version might not be the latest version of this document, if possible use [the french one](https://www.ssi.gouv.fr/entreprise/guide/recommandations-de-securite-relatives-a-tls/)).

### Is it suitable for beginners?

It depends on your definition of a beginner. This software is intended to be used by system administrators with a certain knowledge of their environment. Furthermore, it is also expected to know the bases of the ACME protocol. Let's Encrypt wrote a nice article about [how it works](https://letsencrypt.org/how-it-works/).

### It doesn't work!

ACMEd releases do work properly. Knowing that new users tend to shoot themselves in the foot with hooks, you might want to check those before considering moving away to a different software. Files path and permissions are very common traps, you definitely want to check those.

By the way, don't forget to change the log verbosity using `--log-level trace`.

### Should ACMEd run as root?

Running ACMEd as root is the simplest configuration since you do not have to worry about access rights, especially within hooks (Eg: restart a service).

However, if you are concerned with safety, you should create a dedicated user for ACMEd. Before doing so, please consider the following points:

* Will my services be able to read both the private key and the certificate?
* Will the ACMEd user be able to execute the hooks?

The last one could be achieved using either sudo or Polkit (see the `contrib/polkit` directory).

### Why is there no option to run ACMEd as a specific user or group?

The reason some services has such an option is because at startup they may have to load data only accessible by root, hence they have to change the user themselves after those data are loaded. For example, this is wildly used in web servers so they load a private key, which should only be accessible by root. Since ACMEd does not have such requirement, it should be run directly as the correct user.

### How can I run ACMEd with systemd?

The `contrib/systemd` contains examples of a service file as well as a `sysusers.d` and a `tmpfiles.d` file. Those files might need adjustments in order to work on your system (e.g. paths, user, group,...), but it's probably a good starting point.

### Does ACMEd uses any threading or parallelization?

ACMEd uses a dedicated thread for each endpoint. Certificates of different endpoint can therefore be renewed in parallel while inside each endpoints certificates are renewed sequentially.

### Can I use the same account on different endpoints?

Short answer: Yes, that is possible. However, if you do so, you should be aware that this might eventually hurt the parallelization.

Long answer: Accounts requires to acquire a global lock, therefore an endpoint thread wanting to renew a certificate has to wait that the associated account lock has been released from any other endpoint thread. Since each endpoints renew certificates in a sequential order, they will block on the first certificates which associated account is already in use.

Example:

- 2 accounts: A1 and A2
- 2 endpoints: E1 and E2
- 3 certificates, all requiring to be renewed:
  * C1 on E1 with A1
  * C2 on E1 with A2
  * C3 on E2 with A1

Let's suppose that E1 will renew C1 and C2 in that order. We just launched ACMEd and threads for E1 and E2 starts almost simultaneously. Two cases are possible:

1. E1 acquires the lock for A1 first. E2 is therefore blocked. C1 will be renewed first and only after that C2 and C3 will be renewed in parallel.
2. E2 acquires the lock for A1 first. E1 is therefore blocked. All certificates will be renewed in this sequential order, without any benefit from parallelism: C3, C1 and C2.

There is no way to control neither the sequential certificate renew order inside each endpoint nor the order in which the account locks are acquired.

### Why is RSA 2048 the default certificate key type?

Short answer: it is sufficiently secured, has good performances and is wildly supported.

Before choosing a different algorithm for your certificate's signature, you might want to consider all of those three points.

* For security, you may refer to the table 2 of the [NIST SP 800-57 Part 1](https://csrc.nist.gov/publications/detail/sp/800-57-part-1/rev-5/final).
* For performances, you can launch the following command on your machine: `openssl speed rsa2048 rsa3072 rsa4096 ecdsap256 ecdsap384 ecdsap521 ed25519 ed448`. Your server will be affected by the signature performances and the clients connecting to it will be affected by the verification performances.
* Nowadays, every client support ECDSA. Therefore, unless you have very specific requirements, you can safely use it. At time of writing, EdDSA certificates are not yet supported, but it might become a thing in the future.

Currently, security and client support aren't the main concerns since every possible type of certificates is good enough on those two points. The performances clearly favors ECDSA P-256, Ed25519 and RSA 2048. The later has been chosen as the default because it's the most wildly used as Certification Authorities root and intermediate certificates. This choice may change in favor of ECDSA since Let's Encrypt issued [a full ECDSA certificates chain](https://letsencrypt.org/2020/09/17/new-root-and-intermediates.html).

### Why is ECDSA P-256 the default account key type?

RFC 8555 section 6.2 defines ECDSA P-256 as the only account key type that any ACME servers must implement. It is therefore the best choice for the default value.

### Why can I chose the CSR's digest type but not the certificate's?

Well, you sign the CSR, so obviously you can chose which digest to use. However, the certificate is signed by the certificate authority, so its digest choice is up to your CA. I agree that being able to chose the CSR's digest type is of low importance, sorry if it gave you false hopes about the certificate.
