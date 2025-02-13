# SM
State Machine zkVM prototype extracted from [zkevm-proverjs](https://github.com/0xPolygonHermez/zkevm-proverjs/tree/main/pil).

# Example

### Arithmetization
```
npm run buildrom
npm run buildstoragerom
npm run genstarkstruct
npm run vm
```

### Bottom Layer: FRI Proof

```
../target/release/eigen-zkit stark_prove -s ./build/proof/starkstruct.json \
    -p ./circuits/zkvm.pil.json \
    --o ./circuits/zkvm.const \
    --m ./circuits/zkvm.cm -c circuits/zkvm.circom --i circuits/zkvm.zkin.json
```

### Recursive Layer: FRI Proof

```
../target/release/eigen-zkit compile -p goldilocks -i circuits/zkvm.circom -l node_modules/pil-stark/circuits.gl --O2=full -o /tmp/

node ../starkjs/src/compressor12/main_compressor12_setup.js \
    -r /tmp/zkvm.r1cs \
    -c /tmp/c12.const \
    -p /tmp/c12.pil \
    -e /tmp/c12.exec

node ../starkjs/src/compressor12/main_compressor12_exec.js \
    -w /tmp/zkvm_js/zkvm.wasm  \
    -i circuits/zkvm.zkin.json  \
    -p /tmp/c12.pil  \
    -e /tmp/c12.exec \
    -m /tmp/c12.cm

../target/release/eigen-zkit stark_prove -s ./tools/zkvm.c12.starkstruct.json \
    -p /tmp/c12.pil.json \
    --o /tmp/c12.const \
    --m /tmp/c12.cm -c circuits/circuit.circom --i circuits/circuit.zkin.json
```

### Top Layer: Snark proof
```
bash -x ./tools/gen_final_proof.sh
```

## Generating custom transactions

[README](./tools/gen-input-executor/README.md)
