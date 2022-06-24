# keyword

## Address Mapping

* each evm address is map to a default substrate address
* we create a module `module-evm-accounts` to map a substrate address to a evm address (now we have two private keys, control two accounts)
* flow:
  * assume EVM address e1 is map to SUBSTRATE address s1
  * we need to map e1 to a new address s2
  * transfer all native currency from s1 to s2
  * link e1 to s2
  * TODO: transfer other assets, not only native currency

## Fork pallets

* fork from official substrate repo: `pallets`
* fork from other: `modules`