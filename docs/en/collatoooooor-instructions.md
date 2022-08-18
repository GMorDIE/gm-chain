# How to set up a GM Parachain Collator

Make sure you are logged in as root, otherwise:

``sudo su -``

Install some required stuff:

``curl https://sh.rustup.rs -sSf | sh`` 
(choose option 1 - Proceed with installation (default))
You may need to restart your system before the next steps.

 ``rustup install nightly-2021-11-02``

 ``rustup target add wasm32-unknown-unknown --toolchain nightly-2021-11-02``

 ``apt install cmake git clang libclang-dev``
Type Y to proceed.

 Clone the repo:

 ``git clone https://github.com/GMorDIE/gm_chain``

 Type the following command, and make sure you can see "gm-chain", if not then you likely did something wrong.
 
  ``ls``

 Move the directory to a new directory:

 ``mv gm_chain /usr/local/bin``

 Check that the directory has been moved:

 ``cd /usr/local/bin``

 ``ls``

 The "gm_chain" directory should show up here.

 ``cd``

 ``chown root:root /usr/local/bin/gm_chain``

 You need to compile the code, this will take quite a while depending on your system (30+ minutes is normal):

 ``cd /usr/local/bin/gm_chain``

 ``cargo build --release``

Mid-way through compiling, you likely need to enter this command when prompted:

``rustup target add wasm32-unknown-unknown``

Resume compiling:

``cargo build --release``

Nagivate to this directory:

``cd``

``cd /etc/systemd/system``

Create a new file:

``sudo nano gm_chain-collator.service``

Within that file, paste in the following:

``[Unit]``<br/>
``Description=GM_Chain Validator``<br/>
``After=network-online.target``<br/>
``Wants=network-online.target``<br/>

``[Service]``<br/>
``User=root``<br/>
``Group=root``<br/>
``ExecStart=/usr/local/bin/gm_chain/target/release/gm-chain-node \``<br/>
``--base-path /var/lib/gm_chain \``<br/>
``--collator \``<br/>
``--force-authoring \``<br/>
``--name "YOUR-COLLATOR-NAME-HERE" \``<br/>
``--chain /usr/local/bin/gm_chain/res/kusama/kusama-parachain-live-raw.json \``<br/>
``--port 30333 \``<br/>
``--bootnodes "/ip4/136.243.93.11/tcp/30333/p2p/12D3KooWHr2Qt2kfhR3YvcruVGfTrLcDG4AfuCmu7qq6n6w1Dtcn" \``<br/>
``-- \``<br/>
``--execution wasm \``<br/>
``--chain kusama \``<br/>
``--database=RocksDb \``<br/>
``--unsafe-pruning \``<br/>
``--pruning=1000 \``<br/>
``--port 30343``<br/>
``Restart=on-failure``<br/>

``[Install]``<br/>
``WantedBy=multi-user.target``

Then ctrl + s then ctrl + x to save & exit that file.

Note: If you can't peer with parachain collators change bootnodes to:

``--bootnodes "/ip4/149.102.128.37/tcp/30333/p2p/12D3KooWJqRDxZM7CeJ8ivpbLWSzmEe3AyEzo2Je9Ew9Mnaa9T1j" \``

Before starting the node, create the base-path folder and give it the necessary permissions & ownership:

``cd``

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

Enter y to continue if prompted.

``rm -rf /var/lib/gm_chain/polkadot/chains/ksmcc3/*``

``curl -o - -L https://ksm-rocksdb.polkashots.io/snapshot | lz4 -c -d - | tar -x -C /var/lib/gm_chain/polkadot/chains/ksmcc3``

Once that's downloaded, we need to make sure you add your account to the collator, I would strongly reccomend making a new account for that... go to polkadot.js.org, and make a new account but save the "raw seed", and not the mnemonic.

``/usr/local/bin/gm_chain/target/release/gm-chain-node key insert --base-path /var/lib/gm_chain --chain /usr/local/bin/gm_chain/res/kusama/kusama-parachain-live-raw.json --scheme Sr25519 --suri "your_private_key(RAW SEED)_here" --password-interactive --key-type aura``

Note: see more here if you want to double check the above https://docs.substrate.io/tutorials/get-started/trusted-network/#add-keys-to-the-keystore

Now we can start the the service again:

``systemctl start gm_chain-collator.service``

Now we need to rotate keys and set our keys on chain.

Ensure the collator is running, or this step won't work:

``curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' http://127.0.0.1:9933/``

You will be greeted with an output that looks like:

``{"jsonrpc":"2.0","result":"0xprivate_key_will_be_here0","id":1}``

"result":"**0x_private_key_will_be_here0**" is what we are interested in.

You need to make sure that you have a Polkadot/Substrate account set up, here's some videos in case you don't know how to do that:

1. Polkadot JS Video  https://www.youtube.com/watch?v=dG0DP9vayPY    https://www.youtube.com/watch?v=BpTQBAyFvEk

2. Talisman Video   https://docs.talisman.xyz/talisman/talisman-initiation/setup-a-talisman-wallet  

Now that you have made an account using one of those extensions, head on over to the GM Parachain section of Polkadot JS: 

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama.gmordie.com#/extrinsics

Ensure that you are in the Developer pallet (the top header), and navigate to the extrinsics section in the drop down.

Note: If you are doing this before the token distribution event, please ping the GM Intern in our Discord, and I will send you some $FREN so you can send the extrinsics.

In the "using the selected account field" select the account you just made for the collator.<br/>
In the "submit the following extrinsic field" select "session".<br/>
In the next field (to the right), select "setKeys(keys, proof)".<br/>
In the "keys:" field, paste in your **0x_private_key_will_be_here0** from your node.<br/>
In the "proof" field, type in " 0 ".

Submit the transaction.

Congratulations, you should now be onboarded as a collator.
