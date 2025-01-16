# Kris Facts
Prototyping serving a TUI over SSH to serve as a template for future projects.

## Description
By leveraging [russh](https://github.com/Eugeny/russh) and [ratatui](https://github.com/ratatui/ratatui), this project serves to act as a proof of concept for serving TUIs over SSH. `russh` launches an ssh server that listens on the specified port and address. Once it establishes a connection, it creates a psuedoterminal to serve to the user using the `ratatui` widget defined.

### How to Access
1. Create a ssh key-pair on your machine (Kris Fact's currently leverages public key authentication, and accepts no fallbacks)
2. SSH into port 2222 using the URL `krisfacts.jacobwaldrip.com`
  a. `ssh krisfacts.jacobwaldrip.com -p 2222`
