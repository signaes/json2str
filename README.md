# json2str

Some times all you need is getting a quoted string from a JSON file, for this you can `cargo install json2str`, and then supposing you have a json file named `sample.json` with this content:

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
