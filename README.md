# A wasm, webgl 3D molecule viewer 

*Note: Colcothar is a chemical name for rust!*

Colcothar is filling a small gap for 3d molecule viewers. For those who value new technologies and (somewhat, working on it) fast load times, colco was built for you!

Colco is a single wasm binary that draws a 3d movable molecule in your browser via [glow](https://crates.io/crates/glow), [stdweb](https://crates.io/crates/stdweb), and [glam](https://crates.io/crates/glam). It achieves (somewhat, working on it) fast loadtimes by integrating a build step that loads (via [tobj](https://crates.io/crates/tobj)) all 3d model data (rods and spheres) into u8 slices, perfect for webgl consumption.

## File Support

Currently, only .mol file output from [rdkit](https://www.rdkit.org/) is supported. You also need to embed and include hydrogen in the output.

### RDkit .mol files

Simply run these two functions before running `Chem.MolToMolBlock`:

1. [AllChem.EmbedMolecule](https://www.rdkit.org/docs/source/rdkit.Chem.AllChem.html#rdkit.Chem.oAllChem.ConstrainedEmbedhttps://www.rdkit.org/docs/source/rdkit.Chem.rdDistGeom.html?highlight=embedmolecule#rdkit.Chem.rdDistGeom.EmbedMolecule)
2. [Chem.AddHs](https://www.rdkit.org/docs/source/rdkit.Chem.AllChem.html#rdkit.Chem.AllChem.ConstrainedEmbed)

## Use in Javascript

Initialize colco like this:

```
let rdkitMolOutput = "";

Rust.colco.then((colco)=>colco.initialize(
  'colco-viewer', // Your canvas ID
  rdkitMolOutput, // Output from rdkit
  { atom_size: 2.0, bond_size: 0.5 } // Rendering settings
));
```

## How to Build

Building a `.wasm` file is easy. Colco uses [cargo-web](https://github.com/koute/cargo-web), sort-of like webpack for stdweb in rust. Alternatively, use the `npm` scripts in `package.json`, however this is meant for publishing to npmjs.com.

### Native (in-dev...)

TBD

### Web

`cd` to `colco` directory

Currently only stdweb is supported. To run with stdweb:

```shell
cargo web start --no-default-features --features stdweb --target wasm32-unknown-unknown
```

To generate a new constants.rs file, you need to enable the build script. Like this:

```shell
env GENERATE_CONSTANTS=1 cargo web start --no-default-features --features std_web --target wasm32-unknown-unknown
```

To minify and deploy to static files for production, run:

```shell
cargo web deploy --release --no-default-features --features stdweb --target wasm32-unknown-unknown
```
