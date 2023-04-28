# Config Parse
A `rust` app to extract public information from your config files while keeping secret information
(passwords, keys, etc) hidden. It also checks multiple config files to assert that they include the same keys.

Currently only for `.yaml` and `.properties` files.

# Usage
## With a configuration file
The app is configured through the `config_parser.toml` file.
- `files`: Paths of the files that will be parsed.
- `keys`: `Array` of `String` types of the keys that will be extracted. Separated by dot notation (e.g.
  `auth.client.key`)

## With CLI Arguments
Alternatively, you can supply the files and keys with CLI arguments:

```
config-parse --cli --files test/1.yaml --files test/2.yaml --keys one.big.cascade --keys url
```

# What's next
- Support other file types (`.toml`, ...)
