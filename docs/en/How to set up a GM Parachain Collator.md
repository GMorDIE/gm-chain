# How to set up a GM Parachain Collator

Make sure you are logged in as root, otherwise:

``sudo su u``

Install some required stuff:

``curl https://sh.rustup.rs -sSf | sh`` 
(choose option 1 - Proceed with installation (default))
You may need to restart your system before the next steps.

 ``rustup install nightly-2021-11-02``

 ``rustup target add wasm32-unknown-unknown --toolchain nightly-2021-11-02``

 ``sudo apt install cmake git clang libclang-dev``
Type Y to proceed.

 Clone the repo:

 ``git clone https://github.com/GMorDIE/gm_chain``

 Type the following command, and make sure you can see "gm-chain", if not then you likely did something wrong.
 
  ``ls``

 Move the directory to a new directory:

 ``sudo mv gm_chain /usr/local/bin``

 Check that the directory has been moved:

 ``cd /usr/local/bin``

 ``ls``

 The "gm_chain" directory should show up here.

 ``cd``

 ``sudo chmod +x /usr/local/bin/gm_chain``

 ``sudo chown root:root /usr/local/bin/gm_chain``

 You need to compile the code, this will take quite a while depending on your system (30+ minutes is normal):

 ``cd /usr/local/bin/gm_chain``

 ``cargo build --release``

Mid-way through compiling, you likely need to enter this command when prompted:

``rustup target add wasm32-unknown-unknown``

Resume compiling:

``cargo build --release``

Nagivate to this directory:

``cd /etc/systemd/system``

Create a new file:

``sudo nano gm_chain-collator.service``

Within that file, paste in the following:

``[Unit]``

``Description=GM_Chain Validator``

``After=network-online.target``

``Wants=network-online.target``

``[Service]``

``User=root``

``Group=root``

``ExecStart=/usr/local/bin/gm_chain/target/release/gm-chain-node \``

``--base-path /var/lib/gm_chain \``

``--collator \``

``--force-authoring \``

``--name "YOUR-COLLATOR-NAME-HERE" \``

``--chain /usr/local/bin/gm_chain/res/kusama/kusama-parachain-live-raw.json \``

``--port 30333 \``

``--bootnodes "/ip4/136.243.93.11/tcp/30333/p2p/12D3KooWHr2Qt2kfhR3YvcruVGfTrLcDG4AfuCmu7qq6n6w1Dtcn" \``

``-- \``

``--execution wasm \``

``--chain kusama \``

``--database=RocksDb \``

``--unsafe-pruning \``

``--pruning=1000 \``

``--port 30343``

``Restart=on-failure``

``[Install]``

``WantedBy=multi-user.target``

Then ctrl + s then ctrl + x to save & exit that file.

Before starting the node, create the base-path folder and give it the necessary permissions & ownership:

``mkdir /var/lib/gm_chain``

Let's start the collator:

``chown root:root /var/lib/gm_chain``

``sudo systemctl start gm_chain-collator.service``

Now, let's check that the chain is running

``systemctl status gm_chain-collator.service``

If you get no error messages, then you should be good... check if your node appears here (from your browser):

``https://telemetry.polkadot.io/#list/0x19a3733beb9cb8a970a308d835599e9005e02dc007a35440e461a451466776f8``

Now, let's sync with Kusama, this process will take a long time, depending on your download speed (it needs to download something like 130 gb) if you don't do this step it will likely take even longer, which isn't ideal of course:

``systemctl stop gm_chain-collator.service``

``ls /var/lib/gm_chain``

You should see "chains" and "polkadot" directories. 

``apt install curl lz4 tar``

``rm -rf /var/lib/gm_chain/polkadot/ksmcc3/*``

``curl -o - -L https://ksm-rocksdb.polkashots.io/kusama-14002413.RocksDb.tar.lz4 | lz4 -c -d - | tar -x -C /var/lib/gm_chain/polkadot/chains/ksmcc3``

Once that's downloaded, we need to make sure you add your account to the collator, I would strongly reccomend making a new account for that... go to polkadot.js.org, and make a new account but save the "raw seed", and not the mnemonic.

``/usr/local/bin/gm_chain/target/release/gm-chain-node key insert --base-path /var/lib/gm_chain --chain /usr/local/bin/gm_chain/res/kusama/kusama-parachain-live-raw.json --scheme Sr25519 --suri "your_private_key(RAW SEED)_here" --password-interactive --key-type aura``

Note: see more here if you want to double check the above https://docs.substrate.io/tutorials/get-started/trusted-network/#add-keys-to-the-keystore

Now we can start the the service again:

``systemctl start gm_chain-collator.service``





