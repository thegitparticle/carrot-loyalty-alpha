const anchor = require("@project-serum/anchor");
const assert = require("assert");
const bs58 = require("bs58");

describe("carrot-loyalty-alpha", () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace.CarrotLoyaltyAlpha;

	const brand = anchor.web3.Keypair.generate();

	it("creating a new brand account", async () => {
		// generate a keypair to act as new's brand account

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

		// console.log(brandAccount);

		assert.equal(
			brandAccount.brandAddress.toBase58(),
			program.provider.wallet.publicKey.toBase58()
		);
		assert.equal(brandAccount.brandName, "supreme");
	});

	it("creating a new loyalty account", async () => {
		// generate a keypair to act as new's consumer account
		const consumer = anchor.web3.Keypair.generate();
		const loyalty = anchor.web3.Keypair.generate();

		const signature = await program.provider.connection.requestAirdrop(
			consumer.publicKey,
			1000000000
		);
		await program.provider.connection.confirmTransaction(signature);

		console.log("loyalty and bump found");

		let score = new anchor.BN(711);
		let level = new anchor.BN(3);

		console.log("score and level converted to BigNumber");

		await program.rpc.createLoyalty(
			brand.publicKey,
			"supreme",
			score,
			level,
			{
				accounts: {
					loyalty: loyalty.publicKey,
					authorityConsumer: consumer.publicKey,
					systemProgram: anchor.web3.SystemProgram.programId,
				},
				signers: [consumer, loyalty],
			}
		);

		console.log("rpc call sent out");

		const loyaltyAccount = await program.account.loyalty.fetch(
			loyalty.publicKey
		);

		console.log(loyaltyAccount);

		assert.equal(
			loyaltyAccount.brandAddress.toBase58(),
			brand.publicKey.toBase58()
		);
		assert.equal(loyaltyAccount.brandName, "supreme");
	});
});
