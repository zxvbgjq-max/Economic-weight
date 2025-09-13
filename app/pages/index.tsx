import { useState } from "react";
import { PublicKey, Connection, clusterApiUrl } from "@solana/web3.js";
import { Program, AnchorProvider, web3 } from "@project-serum/anchor";
import idl from "../idl/game_assets.json";

const programID = new PublicKey("GameAssEt11111111111111111111111111111111111");
const network = clusterApiUrl("devnet");

export default function Home() {
  const [log, setLog] = useState("Ready");
  // NOTE: for simplicity this demo triggers RPCs without wallet integration.
  const mintNft = async () => {
    setLog("Minting NFT... (demo)");
    alert("This demo is a template. Connect a wallet and implement provider to call RPCs.");
  };
  return (
    <div style={{padding:40}}>
      <h1>GameAssetX Demo</h1>
      <p>Buttons below emulate actions. To make them work connect wallet and deploy Anchor program to devnet.</p>
      <div style={{display:'flex',gap:12,marginTop:20}}>
        <button onClick={mintNft} style={{padding:12,borderRadius:8,background:'#7c3aed',color:'white'}}>Mint NFT Item</button>
        <button onClick={mintNft} style={{padding:12,borderRadius:8,background:'#06b6d4',color:'white'}}>Mint Land</button>
        <button onClick={mintNft} style={{padding:12,borderRadius:8,background:'#f97316',color:'white'}}>Create Currency</button>
        <button onClick={mintNft} style={{padding:12,borderRadius:8,background:'#ef4444',color:'white'}}>Mint Achievement</button>
      </div>
      <pre style={{marginTop:20,background:'#041022',padding:12,borderRadius:8}}>{log}</pre>
    </div>
  );
}
