import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CarrotLoyaltyAlpha } from "../target/types/carrot_loyalty_alpha";

describe("carrot-loyalty-alpha", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CarrotLoyaltyAlpha as Program<CarrotLoyaltyAlpha>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
