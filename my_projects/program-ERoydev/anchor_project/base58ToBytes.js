const bs58 = require('bs58');
const fs = require('fs');

const base58Key = '....put private key here in base58 format....';
const secretKey = bs58.decode(base58Key);
fs.writeFileSync('wallet.json', JSON.stringify(Array.from(secretKey)));