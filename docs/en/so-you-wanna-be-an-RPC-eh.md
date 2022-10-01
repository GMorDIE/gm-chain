# Setting up an RPC
So, you wanna build an RPC and don't know where to begin? Well, this guide should hopefully help you - but be warned, there is tonnes of room for improvement, and this guide does not contain best practices at all.

>huge thanks to [bLd](https://twitter.com/bLd77)  and the Astar team for helping with this process, you can find more information from him here: https://docs.astar.network/docs/nodes/archive-node/.

I would strongly reccomend that you become familiar with the process of [setting up a GM Collator](https://github.com/GMorDIE/gm-chain/blob/main/docs/en/collatoooooor-instructions.md) before attempting to set up an RPC node. *Note: the guide on how to set up a collator node is equally as basic as this guide and does not contain best practices*.

Again, this guide assumes that you have already [set up a GM Collator node](https://github.com/GMorDIE/gm-chain/blob/main/docs/en/collatoooooor-instructions.md).

First, connect to your server and ensure that it has the latest updates:

``sudo apt-get update``

``sudo apt-get upgrade``

Enter Y to continue.

``sudo apt install -y adduser libfontconfig1``

Next, we need to edit the service file, **this assumes you used the same file path as in the [Collator setup instructions](https://github.com/GMorDIE/gm-chain/blob/main/docs/en/collatoooooor-instructions.md):**

``nano /etc/systemd/system/gm_chain-collator.service``

``[Unit]``<br/>
``Description=GM_Chain Validator``<br/>
``After=network-online.target``<br/>
``Wants=network-online.target``<br/>

``[Service]``<br/>
``User=root``<br/>
``Group=root``<br/>

``ExecStart=/usr/local/bin/gm_chain/target/release/gm-chain-node \``<br/>
``  --pruning=archive \``<br/>
``  --rpc-cors all \``<br/>
``  --name "YOUR_NODE_NAME_HERE" \``<br/>
``  --chain /usr/local/bin/gm_chain/res/kusama/kusama-parachain-live-raw.json \``<br/>
``  --base-path /var/lib/gm_chain \``<br/>
``  --bootnodes "/ip4/149.102.128.37/tcp/30333/ws/p2p/12D3KooWJqRDxZM7CeJ8ivpbLWSzmEe3AyEzo2Je9Ew9Mnaa9T1j" \``<br/>
``  -- \``<br/>
``  --execution Wasm \``<br/>
``  --chain kusama \``<br/>
``  --database=RocksDb \``<br/>
``  --unsafe-rpc-external \``<br/>
``  --ws-external``<br/>

``Restart=always``<br/>
``RestartSec=10``<br/>

``[Install]``<br/>
``WantedBy=multi-user.target``

You will now face the first issue, when you start your node you will get some error such as "incompatible pruning modes" - this is due to us previously using a pruned version of the GM blockchain, where as now we are running an archive node, which needs all of the chains history, not a pruned version. So let's delete our old chain:

``systemctl daemon-reload``

``systemctl stop gm_chain-collator.service``

``rm -rf /var/lib/gm_chain/chains/its_a_lifestyle``

``systemctl start gm_chain-collator.service``

Now, check that everything is okay (check you have connected peers on both relaychain and parachain for example):

``journalctl -u gm_chain-collator.service -f``

Then ctrl + C to exit out of that when you are happy.

Note, if your node doesn't have any parachain peers, try changing the bootnode in the service file we just edited to 

``--bootnodes "/ip4/136.243.93.11/tcp/30333/ws/p2p/12D3KooWHr2Qt2kfhR3YvcruVGfTrLcDG4AfuCmu7qq6n6w1Dtcn" \``

If everything is working normally, for example you have relay chain and parachain peers, and your node [appears on the telemetry page](https://telemetry.polkadot.io/#/0x19a3733beb9cb8a970a308d835599e9005e02dc007a35440e461a451466776f8), then let's proceed to the next steps.

Now, we want to set up a Nginx Server. This guide assumes that you have a domain name and have added your subdomain as a Record, this is typically done in your website / DNS settings.

 We need to install Nginx and Certbot:

``sudo apt-get install nginx snapd``

Enter Y to continue.

``sudo snap install core``

`` sudo snap refresh core``

``sudo snap install --classic certbot``

``sudo ln -s /snap/bin/certbot /usr/bin/certbot``

Now we want to create & enable the site (note, in this section replace ${SUB_DOMAIN} with your own subdomain, for example gmisawesome.gmordie.com)

``cd /etc/nginx/sites-available``

``sudo cp default ${SUB_DOMAIN}``

``sudo ln -s /etc/nginx/sites-available/${SUB_DOMAIN} /etc/nginx/sites-enabled/``

Now we should edit the file we just created:

``sudo nano ${SUB_DOMAIN}``

Change the contents of the file to this:

``server {``<br/>
        ``listen 80;``<br/>
    ``listen [::]:80;``<br/>

   `` root /var/www/${SUB_DOMAIN}/html;``<br/>
   `` index index.html index.htm index.nginx-debian.html;``<br/>
``server_name ${SUB_DOMAIN};``<br/>
 ``location / {``<br/>
            ``try_files $uri $uri/ =404;``<br/>
      ``    }``<br/>
    ``}``

Then CTRL+S and CTRL+X to exit out of that file.

Now we need to issue the certbot certificate:

``sudo certbot certonly --nginx``

Enter a contact email and agree to the terms & conditions.

Certbot will issue the SSL certificate into /etc/letsencrypt/live.

Let's switch to HTTPS by editing that file again:

``sudo nano ${SUB_DOMAIN}``

Delete all existing lines and replace with:

``map $http_upgrade $connection_upgrade {``<br/>
    ``default upgrade;``<br/>
   `` '' close;``<br/>
``}``

``server {``<br/>

   `` # SSL configuration``<br/>
   `` #``<br/>
    ``listen 443 ssl;``<br/>
    ``listen [::]:443 ssl;``<br/>

 ``root /var/www/${SUB_DOMAIN}/html;``

 ``server_name ${SUB_DOMAIN};``<br/>
    ``ssl_certificate /etc/letsencrypt/live/${SUB_DOMAIN}/fullchain.pem; # managed by Certbot``<br/>
    ``ssl_certificate_key /etc/letsencrypt/live/${SUB_DOMAIN}/privkey.pem; # managed by Certbot``<br/>
    ``ssl_session_timeout 5m;``<br/>
    ``ssl_protocols SSLv2 SSLv3 TLSv1 TLSv1.1 TLSv1.2;``<br/>
    ``ssl_ciphers   HIGH:!aNULL:!MD5;``<br/>
    ``ssl_prefer_server_ciphers on;``<br/>

``location / {``<br/>
    `` proxy_pass http://localhost:9944;``<br/>
     ``   proxy_pass_request_headers on;``<br/>
     ``   proxy_http_version 1.1;``<br/>
      ``  proxy_set_header Host $host;``<br/>
      ``  proxy_set_header X-Real-IP $remote_addr;``<br/>
      ``  proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;``<br/>
     ``   proxy_set_header Upgrade $http_upgrade;``<br/>
      ``  proxy_set_header Connection $connection_upgrade;``<br/>
       ``  }``<br/>
   `` }``<br/>

Check & restart Nginx:

``sudo nginx -t``

``sudo systemctl restart nginx``

# Adding your RPC to Polkadot JS website for everyone to use

Now, if you want your RPC to show up in the list on Polkadot JS, you need to submit a pull request to the Polkadot JS apps repo, similarly to: https://github.com/polkadot-js/apps/pull/8186/commits

Once your PR is approved, after a few minutes your node should be available on Polkadot JS!

# Troubleshooting Nginx section

There may be some errors when using Nginx, and unfortunately, even though I'm writing this guide, I'm really new to it, so I'll put all the steps I did to fix some issues with nginx, so maybe you can replicate these steps if you have any issues:

Try and use this command:

``certbot install --cert-name ${SUB_DOMAIN}``

If your output contains something like

>Deploying certificate
Could not install certificate
Could not automatically find a matching server block for ${SUB_DOMAIN}. Set the `server_name` directive to use the Nginx installer.
Ask for help or search for solutions at https://community.letsencrypt.org. See the logfile /var/log/letsencrypt/letsencrypt.log or re-run Certbot with -v for more details.

Then you need to:

``sudo rm -r /etc/nginx/sites-enabled``

``sudo mkdir /etc/nginx/sites-enabled``

``sudo ln -s /etc/nginx/sites-available/leemo.gmordie.com /etc/nginx/sites-enabled/``

The output should look like

>root@Ubuntu-2204-jammy-amd64-base /etc/nginx # ls -al /etc/nginx/sites-enabled
total 8

>drwxr-xr-x 2 root root 4096 Sep 29 10:34 .
drwxr-xr-x 8 root root 4096 Sep 29 10:34 ..

>lrwxrwxrwx 1 root root   44 Sep 29 10:34 {SUB_DOMAIN} -> /etc/nginx/sites-available/${SUB_DOMAIN}

Now, lets try issue the certificate again!

``certbot install --cert-name ${SUB_DOMAIN}``

The output of this should be something like:

>root@Ubuntu-2204-jammy-amd64-base /etc/nginx # certbot install --cert-name ${SUB_DOMAIN}

>Saving debug log to /var/log/letsencrypt/letsencrypt.log

>Deploying certificate

>Successfully deployed certificate for {SUB_DOMAIN} to /etc/nginx/sites-enabled/${SUB_DOMAIN}

Let's restart Nginx:

``sudo systemctl restart nginx``

Now, try and see if your RPC is working (remember replace the entirety of ${SUB_DOMAIN} with your domain), and if you can connect and see blocks being produced, your RPC is working!

https://polkadot.js.org/apps/?rpc=wss%3A%2F%2F${SUB_DOMAIN}#/explorer





