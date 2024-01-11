import { Connection, PublicKey, clusterApiUrl, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

const connection = new Connection(clusterApiUrl("devnet"));
console.log("✅ Connected!");

console.log(`LAMPORTS_PER_SOL : ${LAMPORTS_PER_SOL}`)

const keypair = Keypair.generate();
console.log(`✅ Generated keypair!`)
console.log(`The public key is: `, keypair.publicKey.toBase58());
console.log(`The secret key is: `, keypair.secretKey);
console.log(`✅ Finished!`);


// npx esrun index.ts
