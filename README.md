# rzip

RZIP is a small CLI tool created with Rust language to zip/unzip multiple files.

We are using [Clap](https://github.com/clap-rs/clap) to handle command parsing

## Usages

When in the folder of rzip you can use these commands

**For the name of your file when zipping you'll need to add the file extension in your FILENAME (filename.zip)** 

```bash
    cargo run zip <ZIP_FILE_NAME> [FILES]
```

```bash
    cargo run unzip <ZIP_FILE_NAME>
```

## Windows

* You can add the the path to the **rzip** folder in your PATH environnement variable to use 'rzip' command from anywhere

