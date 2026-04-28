packages
crates 
modules


cargo new -> [packages] -> stores crates and the crates can 
                                /           \
                               /             \
                              /               \
                             /                 \
                             Binary Crates          Library Crates
                             [These are executable]  [These ones are not executable but perform specific functions]



crates contains modules , modules allows us to organize chunks of codes and also it has privacy featues public, internal, private shit are handled at this level.


we have something called workspaces these are very large projects that allows us to store interrelated packages inside the same folders


A package must have at least one crate [1 or more] binary crate
A package can have 0 or 1 library crate
