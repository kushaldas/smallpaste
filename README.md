# Rust tool to upload files to your own server like your own pastebin

The configuration file is a YAML file in `~/.smallpaste.yml`.

```YAML
output: "shell.example.com:public_html/"
url_prefix: "https://example.com/volatile/"
```

This is actually a rewrite of a small part of
<https://gitlab.com/anarcat/pubpaste> while trying to learn Rust.

