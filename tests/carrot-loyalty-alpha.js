const anchor = require("@project-serum/anchor");
const assert = require("assert");
const bs58 = require("bs58");

describe("carrot-loyalty-alpha", () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace.CarrotLoyaltyAlpha;

	const brand = anchor.web3.Keypair.generate();
	const consumer = anchor.web3.Keypair.generate();

	const loyalty = anchor.web3.Keypair.generate();

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

		const signature = await program.provider.connection.requestAirdrop(
			consumer.publicKey,
			1000000000
		);
		await program.provider.connection.confirmTransaction(signature);

		let score = new anchor.BN(711);
		let level = new anchor.BN(3);

		await program.rpc.createLoyalty(
			brand.publicKey,
			"supreme",
			score,
			level,
			{
				accounts: {
					loyalty: loyalty.publicKey,
					consumerAddress: consumer.publicKey,
					systemProgram: anchor.web3.SystemProgram.programId,
				},
				signers: [consumer, loyalty],
			}
		);

		const loyaltyAccount = await program.account.loyalty.fetch(
			loyalty.publicKey
		);

		assert.equal(
			loyaltyAccount.brandAddress.toBase58(),
			brand.publicKey.toBase58()
		);
		assert.equal(loyaltyAccount.brandName, "supreme");
		console.log(loyaltyAccount.loyaltyScore.toNumber());
		console.log(loyaltyAccount.loyaltyLevel.toNumber());
	});

	it("can fetch all loyalty accounts", async () => {
		const allLoyaltyAccounts = await program.account.loyalty.all();
		// assert.equal(allLoyaltyAccounts.length, 3);
		console.log(allLoyaltyAccounts[0]);

		const loyaltyAccountToEdit = allLoyaltyAccounts[0];

		let scoreChange = new anchor.BN(1331);

		await program.rpc.updateLoyalty(scoreChange, {
			accounts: {
				loyalty: loyaltyAccountToEdit.publicKey,
				consumerAddress: loyaltyAccountToEdit.account.consumerAddress,
			},
			signers: [consumer],
		});

		console.log("rpc updateloyalty sent");

		const allLoyaltyAccountsAgain = await program.account.loyalty.all();
		// assert.equal(allLoyaltyAccounts.length, 3);
		console.log(allLoyaltyAccountsAgain[0]);
		console.log(allLoyaltyAccountsAgain[0].account.loyaltyScore.toNumber());
		console.log(allLoyaltyAccountsAgain[0].account.loyaltyLevel.toNumber());
	});
});
