// client.ts — Solana Playground
const PROGRAM_ID = new web3.PublicKey("");

function getPDA(user: web3.PublicKey, id: number): web3.PublicKey {
  const buf = Buffer.alloc(8);
  buf.writeBigUInt64LE(BigInt(id));
  const [pda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("memory"), user.toBuffer(), buf],
    PROGRAM_ID
  );
  return pda;
}

function printNota(label: string, n: any) {
  console.log(`\n  📒 ${label}`);
  console.log(`     ID        : ${n.id}`);
  console.log(`     Título    : ${n.title}`);
  console.log(`     Contenido : ${n.content}`);
  console.log(`     Creada    : ${new Date(n.createdAt.toNumber() * 1000).toLocaleString()}`);
}

const user    = pg.wallet.publicKey;
const program = pg.program;

const ID1 = Date.now();
const ID2 = Date.now() + 1;

const pda1 = getPDA(user, ID1);
const pda2 = getPDA(user, ID2);

console.log("👛 Wallet  :", user.toBase58());
console.log("💰 Balance :", await pg.connection.getBalance(user) / web3.LAMPORTS_PER_SOL, "SOL");
console.log("🔑 PDA #1  :", pda1.toBase58());
console.log("🔑 PDA #2  :", pda2.toBase58());

// ══════════════════════════════════════
//  1. CREATE #1
// ══════════════════════════════════════
console.log("\n1️⃣  CREATE nota #1...");
const tx1 = await program.methods
  .createMemory(new BN(ID1), "Lista de compras", "Leche, Pan, Huevos")
  .accounts({ memory: pda1, user, systemProgram: web3.SystemProgram.programId })
  .rpc({ commitment: "confirmed" });
console.log("✅ TX:", tx1);

// ══════════════════════════════════════
//  2. CREATE #2
// ══════════════════════════════════════
console.log("\n2️⃣  CREATE nota #2...");
const tx2 = await program.methods
  .createMemory(new BN(ID2), "Ideas proyecto", "UI, Tests, Deploy")
  .accounts({ memory: pda2, user, systemProgram: web3.SystemProgram.programId })
  .rpc({ commitment: "confirmed" });
console.log("✅ TX:", tx2);

// ══════════════════════════════════════
//  3. READ — fetch directo por PDA
// ══════════════════════════════════════
console.log("\n3️⃣  READ notas creadas...");
const nota1 = await program.account.memory.fetch(pda1);
const nota2 = await program.account.memory.fetch(pda2);
printNota("Nota #1", nota1);
printNota("Nota #2", nota2);

// ══════════════════════════════════════
//  4. UPDATE #1
// ══════════════════════════════════════
console.log("\n4️⃣  UPDATE nota #1...");
const txU = await program.methods
  .updateMemory(new BN(ID1), "Lista ✅", "Leche ✅, Pan ✅, Huevos, Mantequilla (nuevo)")
  .accounts({
    memory: pda1,
    owner:  user,
    user,
    systemProgram: web3.SystemProgram.programId,
  })
  .rpc({ commitment: "confirmed" });
console.log("✅ TX:", txU);

const updated = await program.account.memory.fetch(pda1);
printNota("Nota #1 actualizada", updated);

// ══════════════════════════════════════
//  5. DELETE #2
// ══════════════════════════════════════
console.log("\n5️⃣  DELETE nota #2...");
const txD = await program.methods
  .deleteMemory(new BN(ID2))
  .accounts({ memory: pda2, owner: user, user })
  .rpc({ commitment: "confirmed" });
console.log("✅ TX:", txD);

try {
  await program.account.memory.fetch(pda2);
  console.log("❌ La cuenta sigue existiendo");
} catch {
  console.log("🗑️  Cuenta cerrada y rent devuelto ✅");
}

// ══════════════════════════════════════
//  6. READ FINAL — solo nota #1
// ══════════════════════════════════════
console.log("\n6️⃣  ESTADO FINAL...");
const final1 = await program.account.memory.fetch(pda1);
printNota("Única nota restante", final1);

console.log("\n\n🎉 CREATE ✅ | READ ✅ | UPDATE ✅ | DELETE ✅");
