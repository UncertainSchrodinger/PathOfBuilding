* lua code quality tools for linting and styling
* move python scripts to lua
* automate fetching passive skill trees
    * either include them in the source or pull them on-demand
* properly cache network calls
* cache lua module loading
* figure out why LoadModule is called all over the place
    * why do we first parse the data to lua text code that transforms it to some
      format and then load that file to get the format wanted?
    * maybe it is used for caching?
* improve local development experience
    * you should not have to manually import anything, just cargo run
    * all files should be available that are needed
    * if you need to import then there should be a command for it
* PassiveTree.lua stuff
    * Why does it try to load old trees always?
    * is it mandatory to have trees like 3.18 or just a bug
* simplify path handling
    * what I've done now is just hack shit until it works
    * where to generate new lua scripts
* maybe separate generation from runtime
    * instead of loading trees on every boot generate them on demand
    * maybe even have a CLI version of PoB, which can do the generation
* fix mixed use of LoadModule/require/io.open
    * Likely io.open is used to detect if the file exists
        * for sure this is doable with just loadfile
