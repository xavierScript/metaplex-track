import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MetaplexTrack } from "../target/types/metaplex_track";

import wallet from "../wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createPluginV2,
  createV1,
  transferV1,
  fetchAssetV1,
  mplCore,
  pluginAuthority,
  MPL_CORE_PROGRAM_ID,
} from "@metaplex-foundation/mpl-core";
import {
  base58,
  createSignerFromKeypair,
  generateSigner,
  signerIdentity,
  sol,
} from "@metaplex-foundation/umi";
import { Keypair, PublicKey, Connection } from "@solana/web3.js";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

const umi = createUmi("https://api.devnet.solana.com").use(mplCore());

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(signer));

const asset = generateSigner(umi);

describe("metaplex-track", () => {
  // Configure the client to use devnet instead of local cluster
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");
  const anchorWallet = new anchor.Wallet(Keypair.fromSecretKey(new Uint8Array(wallet)));
  const provider = new anchor.AnchorProvider(connection, anchorWallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  const program = anchor.workspace.MetaplexTrack as Program<MetaplexTrack>;

  const collection = Keypair.generate();
  const authority = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("authority"), collection.publicKey.toBuffer()],
    program.programId
  );

it("Request Airdrop", async () => {
    const maxRetries = 3;
    let retries = 0;
    
    while (retries < maxRetries) {
      try {
        let airdrop1 = await umi.rpc.airdrop(keypair.publicKey, sol(1));
        console.log("Airdrop successful:", airdrop1);
        break;
      } catch (error) {
        retries++;
        console.log(`Airdrop attempt ${retries} failed:`, error.message);
        
        if (retries < maxRetries) {
          console.log(`Retrying in 2 seconds...`);
          await new Promise(resolve => setTimeout(resolve, 2000));
        } else {
          console.log("All airdrop attempts failed, continuing with existing balance");
        }
      }
    }
  });

  it("Create a Collection", async () => {
    console.log("\nCollection address: ", collection.publicKey.toBase58());

    const tx = await program.methods
      .createCollection()
      .accountsPartial({
        user: provider.publicKey,
        collection: collection.publicKey,
        authority: authority[0],
        systemProgram: SYSTEM_PROGRAM_ID,
        mplCoreProgram: new PublicKey(MPL_CORE_PROGRAM_ID),
      })
      .signers([collection])
      .rpc();

    console.log("\nCollection Created! Your transaction signature", tx);
  });

  it("Create an Asset", async () => {
    const result = await createV1(umi, {
      asset: asset,
      name: "My Nft",
      uri: "https://example.com/my-nft",
      plugins: [
        {
          plugin: createPluginV2({
            type: "Attributes",
            attributeList: [
              {
                key: "Ledger",
                value: "Flex",
              },
            ],
          }),
          authority: pluginAuthority("UpdateAuthority"),
        },
      ],
    }).sendAndConfirm(umi);

    console.log(
      "\nAsset minted. Transaction signature: ",
      base58.deserialize(result.signature)[0]
    );
  });

  it("Fetch an Asset", async () => {
    // Wait for 9 seconds before fetching
    await new Promise(resolve => setTimeout(resolve, 9000));
    
    const fetchedAsset = await fetchAssetV1(umi, asset.publicKey);

    console.log("\nAsset fetched:\n", fetchedAsset);
  });

  it("Mint Core Asset", async () => {
    const assetKeypair = Keypair.generate();

    console.log("\nAsset address: ", assetKeypair.publicKey.toBase58());

    const tx = await program.methods
      .mintAsset()
      .accountsPartial({
        user: provider.publicKey,
        mint: assetKeypair.publicKey,
        collection: collection.publicKey,
        systemProgram: SYSTEM_PROGRAM_ID,
        mplCoreProgram: new PublicKey(MPL_CORE_PROGRAM_ID),
      })
      .signers([assetKeypair])
      .rpc();

    console.log("\nYour transaction signature", tx);
  });
});