const anchor = require("@project-serum/anchor");

const main = async () => {
  console.log("🚀 Starting test...");

  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Myepicprojet;
  const tx = await program.rpc.startStuffOff();

  console.log("📝 Your transaction signature", tx);
}