

import {Keypair} from '@solana/web3.js';
let keyPair = Keypair.generate();


console.log(`You have generated a new keypair: ${keyPair.publicKey.toBase58()}[${keyPair.secretKey}]`)