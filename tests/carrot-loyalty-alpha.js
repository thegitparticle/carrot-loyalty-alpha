const anchor = require("@project-serum/anchor");
const assert = require("assert");
const bs58 = require("bs58");

describe("carrot-loyalty-alpha", () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace.CarrotLoyaltyAlpha;

	it("creating a new brand account", async () => {
		// generate a keypair to act as new's brand account
		const brand = anchor.web3.Keypair.generate();

		let brand_logo_sample =
			"https://assets.supremenewyork.com/assets/logo-supreme-7fbf1f6597b0a6a686e03c82c29b8e7d.png";

		await program.rpc.createBrand("supreme", brand_logo_sample, {
			accounts: {
				brand: brand.publicKey,
				brandAddress: program.provider.wallet.publicKey,
				systemProgram: anchor.web3.SystemProgram.programId,
			},
			signers: [brand],
		});

		const brandAccount = await program.account.brand.fetch(brand.publicKey);

		console.log(brandAccount);

		assert.equal(
			brandAccount.brandAddress.toBase58(),
			program.provider.wallet.publicKey.toBase58()
		);
		assert.equal(brandAccount.brandName, "supreme");
	});
});
