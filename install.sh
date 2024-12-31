#!/bin/bash
cargo build
sudo cp $PWD/target/debug/hassium /bin/hassium
echo Done! now just run \'hassium\'!