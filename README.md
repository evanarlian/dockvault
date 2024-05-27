# dockvault
Easily manage multiple docker logins on multiple registries. By default, docker stores logins in plain text, and so does this tool. Use as you see fit.

# installation
Install using cargo.
```bash
cargo install --git https://github.com/evanarlian/dockvault
```
Add shell completion support. Currently only fish is supported.
```bash
dockvault shell fish > ~/.config/fish/completions/dockvault.fish
```

# usage
```bash
# merge current auth data to dockvault
# note: most of dockvault commands will do implicit merge first
# so `merge` is not necessary before `list` and `use`
$ dockvault merge
Merged all `/home/username/.docker/config.json` to `/home/username/.dockvault/config.json`

# list all stored credentials and which are currently used
$ dockvault list
https://index.docker.io/v1/
  * your_username
    other_username

https://another.registry.com
    alice
  * bob

# log in with other account in same registry
$ dockvault use other_username@https://index.docker.io/v1/
Updated docker config to use `https://index.docker.io/v1/` with username `other_username`

# use docker normally
$ docker pull other_username/some_private_image

# delete all saved dockvault data
$ dockvault delete
Deleted /home/username/.dockvault/config.json
```
