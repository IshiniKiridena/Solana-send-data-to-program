import * as web3js from "@solana/web3.js";
import BN from "bn.js";

const connection = new web3js.Connection(web3js.clusterApiUrl("devnet"))

async function main() {
    //take private key and program id from the env
    const key: Uint8Array = Uint8Array.from([])
    const signer: web3js.Keypair = web3js.Keypair.fromSecretKey(key);

    //get from env
    let programId: web3js.PublicKey = new web3js.PublicKey("FWcjdAByTdbtwfFcdT8V8zCEPbTNc9cUH41g4JuwXnTy")

    // Create a constant `data` of type Buffer to hold binary data.
    // A `Buffer` is used to store a sequence of bytes for encoding or transmitting data.
    const data: Buffer = Buffer.from(
        // Converts the elements of a `Uint8Array` into a `Buffer` for efficient byte manipulation.
        Uint8Array.of(
            0, // The first byte is set to `0`. This could serve as a prefix or identifier in the data.

            // Spread operator `...` unpacks the elements of the array returned by the `BN` conversion.
            ...new BN(3) // Creates a Big Number (`BN`) with the value `3`.

                // Converts the Big Number to an array of bytes in little-endian format.
                // - "le" specifies little-endian byte order (least significant byte first).
                // - `8` specifies the byte length of the resulting array (pads with zeros if necessary).
                .toArray("le", 8)
        )
    );

    let transaction: web3js.Transaction = new web3js.Transaction();

    transaction.add(
        new web3js.TransactionInstruction({
            keys: [],
            programId,
            data
        })
    )

    await web3js.sendAndConfirmTransaction(connection, transaction, [signer])
        .then((sig) => {
            console.log("Signature : ", sig)
        })
}

main();