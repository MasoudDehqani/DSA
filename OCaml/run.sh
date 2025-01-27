#!/bin/bash

# Build and execute the entry of the program and then watch for changes
opam exec -- dune exec --watch OCaml
