mkdir $1
cd $1
cargo init
mkdir inputs
touch inputs/input
touch inputs/demo
cp ../template.rs src/main.rs