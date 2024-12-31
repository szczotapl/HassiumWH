#!/bin/bash
cargo build
sudo cp target/debug/hassiumwh /bin/hassium
echo Done! now just run \'hassium\'!