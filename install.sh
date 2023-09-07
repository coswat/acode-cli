#!/bin/bash

curl -L https://github.com/coswat/acode-cli/releases/download/v1.0.5.alpha/acode-cli > acode-cli

chmod +x acode-cli

mv acode-cli ~/.local/bin

echo "Installation Completed ( test binary )"