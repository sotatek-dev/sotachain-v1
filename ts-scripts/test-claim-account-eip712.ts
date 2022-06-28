import { JsonRpcProvider } from "@ethersproject/providers";
import { HttpProvider } from "@polkadot/api";
import { Wallet } from "ethers";

(async () => {
  /// Alice account

  const provider = new HttpProvider('http://127.0.0.1:9933');
  const ethersProvider = new JsonRpcProvider('http://127.0.0.1:9933')

  const salt: string = await provider.send('chain_getBlockHash', ["0"]);
  console.log({ salt });

  const wallet = new Wallet("0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a");
  const address = wallet.address;
  console.log({address});

  const {chainId} = await ethersProvider.getNetwork();
  console.log({chainId})

  const domain = {
    name: "Sota EVM claim",
    version: "1",
    chainId: chainId.toString(),
    salt,
  };

  const types = {
    Transaction: [{ name: "substrateAddress", type: "bytes" }],
  };

  const value = {
    substrateAddress: "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
  };

  const signature = await wallet._signTypedData(domain, types, value);
  console.log({signature});

  await provider.disconnect();
})();