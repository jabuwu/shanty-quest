set -e
./build-wasm.sh
rm -rf ghpages
mkdir ghpages
cp -r .git ghpages/
(cd ghpages && git fetch)
(cd ghpages && git config user.name "ghpages")
(cd ghpages && git config user.email "ghpages")
(cd ghpages && git switch ghpages)
cp -r wasm/* ghpages/
(cd ghpages && git add *)
(cd ghpages && git commit -m "update ghpages")
(cd ghpages && git push -u origin ghpages)