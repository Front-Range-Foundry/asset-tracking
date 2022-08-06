# asset-tracking
An asset tracking application. 
This simple Rust application is a project that uses everything in The Book up to chapter five. It creates a very simple, non-persisting RESTful application that allows its users to: 
- create assets
- update an asset
- delete an asset 
- get an asset
- get all assets. 

Each asset is a struct that consists of only a `name` and `id` field. The library uses the `uuid` package to construct unique IDs. 

## Future State
As we move to chapter 6 of the book - Enums - we may complicate the Asset struct with increasingly specific fields (eg, Asset Typing, etc) 
This will require better code organization, doubtless. 