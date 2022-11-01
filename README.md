## Replacer

#### About
A text snippet replacer written in Rust.

Manage your replacements in `~/.replacer.json`

#### Usage

Create a config file in your home directory: `~/.replacer.json` with the following:
```json
{
  "replacements":[
    {
      "from":"foo","to":"bar","matches":""
    },
    {
      "from":"princess","to":"peach","matches":""
    }
  ]
}
```

Note: Must be valid JSON. A sample config will be created for you if you run the
program without first having one

---
Run the program with: `$ cargo run`
