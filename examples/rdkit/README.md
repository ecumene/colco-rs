# Colco Example - Webpack

This is a more fully realized example, meant for an rdkit implementation.

## Setup

Simply run these couple of commands:

### Start the rdkit docker

```
git clone git@github.com:ecumene/rdkit-aas rdkit-service && cd rdkit-service
sudo docker-compose up -d
```

### Start the webpack dev server

```shell
cd ..
npm i
mkdir assets && cp ./node_modules/colco/dist/colco.wasm assets/
npm run start
```
