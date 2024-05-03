## How to use

1. Choose a master password, say "password"

2. Get sha256 chksum. 
```sh 
echo password | sha256sum
```

3. Set the variable MASTER as the output of the previous command.

4. To hide data run 
```sh
cargo r -- enc "png_name" "secret to hide"
``` 

This will produce a `png_name-output.png`

5. To get back data run 
```sh
cargo r -- dec "png_name-output.png"
```
You'll be prompted for the original master password

## Is the data completely hidden?
No. Anyone with sufficient programming knowledge can easily reverse engineer the encoding algo.
Do not use this to store nuclear launch codes
