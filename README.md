# Config Parse
A `rust` app to extract public information from your config files while keeping secret information
(passwords, keys, etc) hidden. Currently only for `.yaml` files.

# Usage
The app is configured through the `config_parser.toml` file.
- `keys`: `Array` of `String` types of `YAML` keys. Separated by dot notation (e.g.
  `auth.client.key`)
- `dest`: Destination path of the `.csv` file containing the extracted key-value pairs.

# Future Work
- Support other file types (`.property`, `.toml`, ...)
- Support multi-file extraction (e.g. for `dev`, `stage`, `prod` environments)
