import * as anchor from "@project-serum/anchor";
import { Pdas } from "../target/types/pdas";

function shortKey(key: anchor.web3.PublicKey) {
  return key.toString().substring(0, 8);
}

describe("pdas", () => {
  
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Pdas as anchor.Program<Pdas>;

  async function generateKeypair() {
    let keypair = anchor.web3.Keypair.generate();
    const tx = await provider.connection.requestAirdrop(
      keypair.publicKey,
      0.2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(tx);
    //await new Promise( resolve => setTimeout(resolve, 3 * 1000) ); // Sleep 3s
    return keypair;
  }

  async function derivePda(pubkey: anchor.web3.PublicKey) {
    let [pda, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        //pubkey.toBuffer(),
        Buffer.from("new_seed"),
        //Buffer.from(color),
      ],
      program.programId
    );
    return pda;
  }

  async function createLedgerAccount(
    //color: string, 
    pda: anchor.web3.PublicKey, 
    wallet: anchor.web3.Keypair
  ) {
    await program.methods.createLedger()
      .accounts({
        ledgerAccount: pda,
        wallet: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();
  }
  it("An example of PDAs in action", async () => {

  try {
    const wallet = await generateKeypair(); //.then(async (wallet)=>{
    const pda = await derivePda(wallet.publicKey);//.then(async (pda)=>{
    await createLedgerAccount( pda , wallet); 
    console.log(`successfully created PDAs`);

    const data = await program.account.ledger.fetch(pda);

    console.log(`Count: ${data.count} , Balance: ${data.balance}`);
  //} 
  //    );
    //}
  //);  

  } catch (error) {
    console.log(`somthing went wrong ${error}`);
  }
})

});