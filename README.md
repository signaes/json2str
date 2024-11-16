# json2str

Some times all you need is getting a quoted string from a JSON file, for this you can clone this repo, and then:

```
cargo build --release
cp target/release/json2str <to somewhere in you path>
```

Then after reloading the shell, supposing you have a json file named `sample.json` with this content:

```
{
  "test": true
}
```

Running:

```
json2str sample.json
```

Should give you:

```
"{\"test\":true}"
```
