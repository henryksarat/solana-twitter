// export const sendTweet = async (topic, content) => {
//     return {
//         topic,
//         content,
//         author_display: 'B1Af..wtRN',
//         created_at: 'Nov 26, 2021 1:03PM',
//         created_ago: 'just now',
//         timestamp: 1637932868,
//     }
// }


import { web3 } from '@project-serum/anchor'
import { useWorkspace } from '@/composables'
import { Tweet } from '@/models'
// import { PublicKey } from '@solana/web3.js'

// 1. Define the sendTweet endpoint.
export const sendTweet = async (topic, content) => {
    const { wallet, program } = useWorkspace()
  
    // 2. Generate a new Keypair for our new tweet account.
    const tweet = web3.Keypair.generate()

    console.log(tweet.publicKey.toBase58())
    // 3. Send a "SendTweet" instruction with the right data and the right accounts.
    await program.value.rpc.sendTweet(topic, content, {
        accounts: {
            author: wallet.value.publicKey,
            tweet: tweet.publicKey,
            systemProgram: web3.SystemProgram.programId,
        },
        signers: [tweet]
    })

    // sleep(1000);
//     console.log(tweet.publicKey.toBase58())
//     var pub = '5uWe4mPhphXm3pjhaBd36vGmPgLzhzC3tycVnwxd52pD'
//     const programID = new PublicKey(pub)
// 
//     console.log('trying with=' + programID.toBase58())
// 
//     const tweetAccountBefore = await program.value.account.tweet.fetch(programID)
// 
//     var rightnow = new Tweet(pub, tweetAccountBefore)
// 
//     console.log('-----')
//     console.log(rightnow.content)
//     console.log('-----')
    
    // 4. Fetch the newly created account from the blockchain.
    const tweetAccount = await program.value.account.tweet.fetch(tweet.publicKey)
    
    // 5. Wrap the fetched account in a Tweet model so our frontend can display it.
    return new Tweet(tweet.publicKey, tweetAccount)
}

// function sleep(milliseconds) {
//   const date = Date.now();
//   let currentDate = null;
//   do {
//     currentDate = Date.now();
//   } while (currentDate - date < milliseconds);
// }
