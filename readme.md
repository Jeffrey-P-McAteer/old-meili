
# Meili

## Name Etymology

```
Meili is one of the Æsir in Norse mythology.
His name appears to mean "mile-stepper", and if accurate,
could mean that he was a Norse god of travel.
Given the importance of travel in Norse culture,
Meili would then have been an important figure in the Norse pantheon,
but no first-hand accounts of his status are known to exist,
so his rank and function among the Æsir remains a point of conjecture.
```
(https://mythology.wikia.org/wiki/Meili)

## What is Meili?

I wanted to build a p2p communication system with several properties:

 - communication backend agnostic
    - may tx and rx over tcp/udp/ssh/http PUT+GETs etc.
 - zero ambiguity identities
    - all identities must be backed by crypto, ideally hardware tokens or cards
 - flexible message processing
    - after a communication channel is created, messages may be passed to custom programs for processing
 - cross platform
    - we get this for free with `rust` + `mingw-w64-*` + `apple-darwin-osxcross`
 - graphical AND cli interfaces
    - "There are two types of interfaces: good interfaces, and human interfaces"
    - I really want to get experience making a dead-simple cross-platform GUI that does some useful task like talking to peers

## How does one configure Meili?

Meili uses [app_dirs](https://docs.rs/app_dirs/1.2.1/app_dirs/) to use an OS-friendly directory
for storing data in. You can have Meili tell you where it's configuration is located by running:

```bash
./meili --about
```

Which will output something like

```

```

The `meili.toml` file contains comments for each item, and
an example config file is located at [`src/meili.toml`](src/meili.toml).


## How does one use Meili?

```bash
./meili
```

## How does one build Meili?

```bash
python3 build.py
```

The optional environment variables assigned during builds exist:

```
MEILI_BUILD_ADD_DELAYS=1
  This inserts delays where it makes sense to verify tasks execute.
  At the moment it is used to verify winapi::um::wincon::FreeConsole works as expected.

```


# License

The code in this repository is under the GPLv2 license, see `LICENSE.txt` for details.
The auto-upgrade clause has been removed because your legal rights shouldn't have that sort of volatility.
