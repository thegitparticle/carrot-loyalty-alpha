import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CarrotLoyaltyAlpha } from "../target/types/carrot_loyalty_alpha";

describe("carrot-loyalty-alpha", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CarrotLoyaltyAlpha as Program<CarrotLoyaltyAlpha>;

  it("can add a brand to a consumer", async () => {
      // generate a keypair to act as new's brand account
      const brand = anchor.web3.Keypair.generate();

      // rpc call to the program
      // await program.rpc.
  })

});
