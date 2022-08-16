# HOW TO UPDATE A GM PARACHAIN COLLATOR

Note: These instructions explain how to update an existing collator node. For instructions on how to initially set up a collator node, see here: https://github.com/GMorDIE/gm_chain/blob/main/docs/en/collatoooooor-instructions.md 

Note: These instructions include directory file paths which are based on those given here: https://github.com/GMorDIE/gm_chain/blob/main/docs/en/collatoooooor-instructions.md  if you do not explicitly follow the initial set up instructions, you may encounter errors when updating your collator.

This initial version of these instructions will just explain how to clone the repo, recompile, and replace the previous files. These instructions will be periodically updated to include easier / more user friendly options. I'm just an intern... please be gentle..

Ensure you are logged in with root, otherwise ``sudo su -``

First you need to stop the existing service and remove your old directory:

``systemctl stop gm_chain-collator.service``

``cd /usr/local/bin/``

``ls``

Confirm that you see ``gm_chain``

Remove the directory ``rm -r gm_chain``

``ls``

Confirm that gm_chain no longer exists.

Clone the repo again ``git clone https://github.com/GMorDIE/gm_chain``

``ls``

Confirm that you see gm_chain again.

``cd``

``sudo chown root:root /usr/local/bin/gm_chain``

Compile the code:

``cd /usr/local/bin/gm_chain``

``cargo build --release``

Let's start the collator:

``cd``

``chown root:root /var/lib/gm_chain``

``systemctl start gm_chain-collator.service``

Now, let's check that the chain is running:

``systemctl start gm_chain-collator.service``

Alternatively, you can also ``journalctl -u gm_chain-collator.service -f`` to get better diagnostics.

If you get no error messages, then you should be good... check if your node appears here (from your browser):

https://telemetry.polkadot.io/#list/0x19a3733beb9cb8a970a308d835599e9005e02dc007a35440e461a451466776f8


