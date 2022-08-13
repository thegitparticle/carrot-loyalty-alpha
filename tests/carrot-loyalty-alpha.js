const anchor = require("@project-serum/anchor");
const assert = require("assert");
const bs58 = require("bs58");

describe("carrot-loyalty-alpha", () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace.CarrotLoyaltyAlpha;

	it("can add a brand to a consumer", async () => {
		// generate a keypair to act as new's brand account
		const brand = anchor.web3.Keypair.generate();

		let score = new anchor.BN(711);
		let level = new anchor.BN(3);

		// rpc call to the program
		await program.rpc.addBrand("supreme", score, level, {
			accounts: {
				brand: brand.publicKey,
				consumer: program.provider.wallet.publicKey,
				systemProgram: anchor.web3.SystemProgram.programId,
			},
			signers: [brand],
		});

		const brandAccount = await program.account.brand.fetch(brand.publicKey);

		assert.equal(
			brandAccount.consumer.toBase58(),
			program.provider.wallet.publicKey.toBase58()
		);
		assert.equal(brandAccount.brandName, "supreme");
	});
});
