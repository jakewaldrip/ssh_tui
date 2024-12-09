# SSH TUI
Prototyping serving a TUI over SSH to serve as a template for future projects.

## Description
By leveraging [russh](https://github.com/Eugeny/russh) and [ratatui](https://github.com/ratatui/ratatui), this project serves to act as a proof of concept for serving TUIs over SSH. `russh` launches an ssh server that listens on the specified port and address. Once it establishes a connection, it creates a psuedoterminal to serve to the user using the `ratatui` widget defined.

## TODO
* [X] Split the example into separate files for easier extension
* [ ] Expand the widget to implement a simple interactive process
* [ ] Deploy on VPS to prove the concept end to end using an existing owned domain
  * [ ] Ensure we can still ssh into the server normally
  * [ ] Ensure our current deployed applications remain online
