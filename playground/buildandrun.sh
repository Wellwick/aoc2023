# Not very safe, just here for me to run things in the playground quickly
# (yes we are in Linux-land)

# First argument is source file, without the extension. If this compiles, then
# we try to run the build executable, which depends on there being a main method
rustc $1.rs && ./$1
