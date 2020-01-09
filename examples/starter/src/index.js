import colco from 'colco';

let canvas = document.createElement("canvas");
document.getElementsByTagName("body")[0].appendChild(canvas);
canvas.id = "colco-viewer";
canvas.width = 800;
canvas.height = 800;

let molecule = `
     RDKit          2D

 15 15  0  0  0  0  0  0  0  0999 V2000
    3.0000    0.0000    0.0000 N   0  0  0  0  0  0  0  0  0  0  0  0
    1.5000    0.0000    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    0.7500   -1.2990    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
   -0.7500   -1.2990    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
   -1.5000    0.0000    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
   -0.7500    1.2990    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
   -1.5000    2.5981    0.0000 O   0  0  0  0  0  0  0  0  0  0  0  0
    0.7500    1.2990    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    3.7500   -1.2990    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    3.7500    1.2990    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    1.5000   -2.5981    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
   -1.5000   -2.5981    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
   -3.0000    0.0000    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
   -0.7500    3.8971    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    1.5000    2.5981    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
  1  2  1  0
  2  3  2  0
  3  4  1  0
  4  5  2  0
  5  6  1  0
  6  7  1  0
  6  8  2  0
  8  2  1  0
  1  9  1  0
  1 10  1  0
  3 11  1  0
  4 12  1  0
  5 13  1  0
  7 14  1  0
  8 15  1  0
M  END
`;

colco.then((colco) => colco.initialize(
  'colco-viewer', // Your canvas ID
  molecule, // Output from rdkit
  { atom_size: 2.0, bond_size: 0.5 } // Rendering settings
));

