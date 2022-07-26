## Solana Twitter

This is an example Solana project that has a smart contract that writes tweets to the blockchain and read tweets from the block chain. The UI calls this smart contract for writing and reading. 

Right before tweet, about to approve transaction using Phantom:
![This is an image](https://github.com/henryksarat/solana-twitter/blob/main/assets/solana_twitter_one.png)

After tweet was saved via smart contract and fetched to display:
![This is an image](https://github.com/henryksarat/solana-twitter/blob/main/assets/solana_twitter_two.png)


**Running unit tests**

Step 1:

```
solana config set --url localhost
```

Step 2:
```
In Anchor.toml ensure that cluster is equal to localnet, like:

[provider]
cluster = "localnet"
```

Step 3:
```
anchor test
```

**Run the UI against devnet:**
```
anchor run copy-idl

solana config set --url devnet

solana airdrop 2 {public_key}
solana airdrop 2 {public_key}

In Anchor.toml ensure that cluster is equal to devnet, like:
[provider]
cluster = "devnet"

anchor build

anchor deploy

cd app
npm run serve:devnet
```

**How to generate a key and get the address again:**
```
solana-keygen new -o localkey.json
solana address -k localkey.json
```


**How to get the address of the built program:**
```
solana address -k target/deploy/solana_twitter-keypair.json
```


**Possible causes of error when deploying to devnet to try and debug step by step:**

1. Set **cluster** to **devnet**

2. Make a new key and airdrop ~10 sol

```
solana-keygen new -o localkey.json
solana address -k localkey.json

# split up because of rate limiting
solana airdrop 2 {public_key}
solana airdrop 2 {public_key}
solana airdrop 2 {public_key}
solana airdrop 2 {public_key}
solana airdrop 2 {public_key}
```

3. Set the wallet address of the location of the key from #1 in Anchor.toml

4. Delete the fiiles in target/deploy/*

5. execute

```
anchor build
anchor deploy
```

6. Take the public key from the deploy and add to the top of program/solana/src/lib.rs and to the desired programs in Anchor.toml

7. execute 

```
anchor build
anchor deploy
```
