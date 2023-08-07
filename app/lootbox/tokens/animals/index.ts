import * as web3 from "@solana/web3.js";
import {DataV2, createCreateMetadataAccountV3Instruction} from "@metaplex-foundation/mpl-token-metadata"
import {Metaplex, keypairIdentity, bundlrStorage, toMetaplexFile} from "@metaplex-foundation/js";
import * as fs from "fs"
import {initializeKeypair} from "./initializeKeypair.js"
import * as token from  "@solana/spl-token";


const main = async () => {

    const connection = new web3.Connection(web3.clusterApiUrl("devnet"));
    const payer = await initializeKeypair(connection);
    const assets = ["cat", "duck", "monkey", "pig", "fox"];
    const LOOTBOX_PROGRAM_ID = new web3.PublicKey("wPZy7zBjiXQ8w7Q2hQDXzQamv434Q3Xouzjpu5WCPZC");
    const [token_mint_authority] = web3.PublicKey.findProgramAddressSync([Buffer.from("mint_authority")], LOOTBOX_PROGRAM_ID);
    await createMetadataAssets(connection, payer, assets, token_mint_authority);
   
}

const createMetadataAssets = async(connection: web3.Connection, payer: web3.Keypair, assets: string[], authority: web3.PublicKey) => {
    const metaplex = Metaplex.make(connection)
    .use(keypairIdentity(payer))
    .use(bundlrStorage({
        address: "https://devnet.bundlr.network",
        providerUrl: "https://api.devnet.solana.com",
        timeout: 60000,
    }));

    let collection:any = {}

    for(let i = 0; i < assets.length; i++){
        try{
            let image_buffer = fs.readFileSync("./tokens/animals/assets/" + assets[i] + ".png");
            console.log(assets[i] + " image_buffer", image_buffer)
            //buffer to metaplex file for upload
            let image_metaplex_file = toMetaplexFile(image_buffer, `${assets[i]}.png`)

            //upload image into off-chain
            let image_uri = await metaplex.storage().upload(image_metaplex_file)

            //upload metadata into offchain and get the uri
            let {uri} = await metaplex.nfts().uploadMetadata({
                name: assets[i],
                symbol: "Animals",
                description: "Your little pet joining you in the adventure",
                seller_fee_basis_points: 0,
                image: image_uri
            })

            //create mint
            let token_mint = await token.createMint(
                connection,
                payer,
                payer.publicKey,
                payer.publicKey,
                0
            );

            //create PDA for mint to associate with the metadata
            let metadata_pda = metaplex.nfts().pdas().metadata({mint: token_mint})
            
            let dataV2 = {
                name: assets[i],
                symbol: "Animals",
                uri: uri,
                sellerFeeBasisPoints: 0,
                creators: null,
                collection: null,
                uses: null
            } as DataV2

            let ix = createCreateMetadataAccountV3Instruction(
                {
                    metadata: metadata_pda,
                    mint: token_mint,
                    mintAuthority: payer.publicKey,
                    payer: payer.publicKey,
                    updateAuthority: payer.publicKey,
                },
                {
                    createMetadataAccountArgsV3:{
                        data: dataV2,
                        isMutable: true,
                        collectionDetails: null
                    }
                }
            )

            const transaction = new web3.Transaction();
            transaction.add(ix);

            const tx = await web3.sendAndConfirmTransaction(connection, transaction, [payer]);

            console.log(`Metadata uploaded, Mint created : https://explorer.solana.com/tx/${tx}?cluster=devnet`);

            const token_auth_tx = await token.setAuthority(
                connection,
                payer,
                token_mint,
                payer.publicKey,
                token.AuthorityType.MintTokens,
                authority
            )

            console.log(`Token Auth Tx : https://explorer.solana.com/tx/${token_auth_tx}?cluster=devnet`);

            collection[assets[i]] = token_mint.toString();
        }catch(error){
            console.log("ERRO!!:", error);
        }

    }
    
    fs.writeFileSync("./tokens/animals/cache.json", JSON.stringify(collection))

    // createCreateMetadataAccountV3Instruction(
    //     metadata: web3.PublicKey;
    //     mint: web3.PublicKey;
    //     mintAuthority: web3.PublicKey;
    //     payer: web3.PublicKey;
    //     updateAuthority: web3.PublicKey;
    //     systemProgram?: web3.PublicKey;
    //     rent?: web3.PublicKey;
    // )

    // DataV2 = {
    //     name: string;
    //     symbol: string;
    //     uri: string;
    //     sellerFeeBasisPoints: number;
    //     creators: beet.COption<Creator[]>;
    //     collection: beet.COption<Collection>;
    //     uses: beet.COption<Uses>;
    // };

}




main().then((res)=>{
    console.log("Ending")
    process.exit(1)
}).catch((error)=> {
    console.log("Error")
    process.exit(0)
})