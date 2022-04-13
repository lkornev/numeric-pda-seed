import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NumericPdaSeed } from "../target/types/numeric_pda_seed";
const { SystemProgram, PublicKey } = anchor.web3;

describe("numeric-pda-seed", () => {
    anchor.setProvider(anchor.Provider.env());

    const program = anchor.workspace.NumericPdaSeed as Program<NumericPdaSeed>;

    const seed_key = 0;

    it("It creates a PDA with a numbric seed and accesses it!", async () => {
        const [pda, bump] = await PublicKey.findProgramAddress(
            [
                Uint8Array.from([seed_key]),
            ],
            program.programId
        );

        await program.rpc.create(seed_key, bump, {
            accounts: {
                authority: program.provider.wallet.publicKey,
                accountNumericSeed: pda,
                systemProgram: SystemProgram.programId
            },
        });

        await program.rpc.access(seed_key, {
            accounts: {
                accountNumericSeed: pda,
            },
        });
    });
});
