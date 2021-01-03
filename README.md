# Welcome to lorry
# A project inspired by SwiftPM
## What if you could write Cargo.tomls in Rust
# Like this!!
`
   lorry::Program::new()
    .name("spectesta")
    .author("jakob Neufeld")
    .version("1.1.0")
    .gen();
`

## **This project is in beta**
## I would love to get feedback

---
### Right know I have these fields implemented
- [x]  author
- [x]  version
- [x]  name
- [X]  dependencies
### If you need an extra field, just add it.
### lorry only touches the fields in needs to
### lorry keeps all of your old Cargo.toml safe :-)
# How Does it work
---
It uses a builder pattern api.
When you call `gen()`.
It generates the toml and then saves it to disk.

Thanks to toml edit, only the fields that are necessary are changed

---
# How to add it
1. Make a build script
2. Add the build_dependencie lorry
3. Look at the sample code above
4. Paste it into the build script
5. run cargo check to generate Cargo.toml
6. If you are using vscode with rust- analyzer it works automatically (because rust analyzer runs cargo check every time)
---
# Lorry cli is coming soon
---
# Why
---
1.  You can  make a dynamic cargo toml, for example, the version is automatically updated with env vars.
2. Just because it is cool
3. It has IDE completion which helps you write cleaner cargo toml files
---
# Please do not use this in big projects
# Nothing is garanteed stable or working
---
Contact me at
jakob.n.neufeld@gmail.com
or make an issue on the github repo
---
Thanks
