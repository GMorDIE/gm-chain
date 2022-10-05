# How to set up a GM Parachain Collator

Make sure you are logged in as a user with root privileges, otherwise:

``sudo su -``

Create a new service user to run your collator service:
`sudo useradd --no-create-home --shell /usr/sbin/nologin gmcollator`

Install some required stuff:

``curl https://sh.rustup.rs -sSf | sh``
(choose option 1 - Proceed with installation (default))
You may need to restart your system before the next steps.
```
rustup install nightly-2021-11-02
rustup target add wasm32-unknown-unknown --toolchain nightly-2021-11-02
apt install cmake git clang libclang-dev
```
Type Y to proceed.

Clone the repo:

 ``git clone git@github.com:GMorDIE/gm-chain.git``

Type the following command, and make sure you can see "gm-chain", if not then you likely did something wrong.

  ``ls -la``

Enter the repo and check out the most recent tagged release of code (https://github.com/GMorDIE/gm-chain/releases)

```
cd gm_chain
git checkout v1.1.3
```

You need to compile the code, this will take quite a while depending on your system (30+ minutes is normal):
 ``cargo build --release``

Mid-way through compiling, you likely need to enter this command when prompted:

``rustup target add wasm32-unknown-unknown``

Resume compiling:

``cargo build --release``

Move the node executable to `usr/local/bin`, make it executable, and change ownership to our `gmcollator` service user:
```
sudo mv ~/gm_chain/target/release/gm-chain-node /usr/local/bin
sudo chmod +x /usr/local/bin/gm-chain-node
sudo chown gmcollator:gmcollator /usr/local/bin/gm-chain-node
```

Create the base-path folder, copy the gm_chain "chainspec" into it, and give it the necessary permissions & ownership:
```
sudo mkdir /var/lib/gm_chain
sudo cp ~/gm_chain/res/kusama/kusama-parachain-live-raw.json /var/lib/gm_chain/kusama-parachain-live-raw.json
sudo chown -R gmcollator:gmcollator /var/lib/gm_chain
```

Create a systemd service file to run your collator (and automatically restart it):

`sudo nano /etc/systemd/system/gm_chain-collator.service`

Within that file, paste in the following:
```
[Unit]
Description=GM_Chain Collator
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=gmcollator
Group=gmcollator
ExecStart=/usr/local/bin/gm-chain-node \
  --base-path /var/lib/gm_chain \
  --collator \
  --force-authoring \
  --name "YOUR-COLLATOR-NAME-HERE" \
  --chain /var/lib/gm_chain/kusama-parachain-live-raw.json \
  --port 30333 \
  --telemetry-url "wss://telemetry.polkadot.io/submit 0" \
  --bootnodes "/ip4/136.243.93.11/tcp/30333/ws/p2p/12D3KooWHr2Qt2kfhR3YvcruVGfTrLcDG4AfuCmu7qq6n6w1Dtcn" \
  -- \
  --execution wasm \
  --chain kusama \
  --database=RocksDb \
  --unsafe-pruning \
  --pruning=1000 \
  --port 30343

Restart=always
RestartSec=120
[Install]
WantedBy=multi-user.target
```

Then ctrl + s then ctrl + x to save & exit that file.

Note: If you can't peer with parachain collators change bootnodes to:

``--bootnodes "/ip4/149.102.128.37/tcp/30333/ws/p2p/12D3KooWJqRDxZM7CeJ8ivpbLWSzmEe3AyEzo2Je9Ew9Mnaa9T1j" \``

Let's start the collator:

`sudo systemctl daemon-reload && sudo systemctl enable gm_chain-collator && sudo systemctl start gm_chain-collator.service`

Now, let's check that the chain is running

``sudo systemctl status gm_chain-collator.service``

If the service indicates it's "running" and you see no errors, you should be ok. If not, you can debug using one of the following:
`sudo journalctl -fu gm_chain-collator`
`sudo systemctl status --full --lines=100 gm_chain-collator`

Check if your node appears here (from your browser):

``https://telemetry.polkadot.io/#list/0x19a3733beb9cb8a970a308d835599e9005e02dc007a35440e461a451466776f8``

Syncing the Kusama relaychain will take a long time, depending on your download speed (it needs to download something like 130 gb via P2P). If you'd like to accelerate that process you can download a snapshot of the Kusama relaychain to start with:

``sudo systemctl stop gm_chain-collator.service``

``ls /var/lib/gm_chain``

You should see "chains" and "polkadot" directories.

``sudo apt install curl lz4 tar``

Enter y to continue if prompted.

``sudo rm -rf /var/lib/gm_chain/polkadot/chains/ksmcc3/*``

``sudo curl -o - -L https://ksm-rocksdb.polkashots.io/snapshot | sudo lz4 -c -d - | sudo tar -x -C /var/lib/gm_chain/polkadot/chains/ksmcc3``

Once that's downloaded, we need to make sure you add your account to the collator, I would strongly reccomend making a new account for that... go to polkadot.js.org, and make a new account but save the "raw seed", and not the mnemonic.

``/usr/local/bin/gm-chain-node key insert --base-path /var/lib/gm_chain --chain /var/lib/gm_chain/kusama-parachain-live-raw.json --scheme Sr25519 --suri "your_private_key(RAW SEED)_here" --password-interactive --key-type aura``

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
