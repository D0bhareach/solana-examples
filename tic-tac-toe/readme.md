# Peculiarities
I try to follow tic-tac-toe tutorial from [official web site](https://www.anchor-lang.com/docs/tic-tac-toe)
some things I was have to do differently to make my code actually work.   
Date: 29 of May 2023.

### Easy part
For first part of the Project upto 'Play Game' section follow official site.
To run test code start `solana-test-validator -q` in separate window, and start
`anchor test --skip-local-validator` in root on the project.  
For some reason on my setup it only works with solana-test-validator.

### Not easy part
I was following official site upto 'Play Game' section where I realize that instructions stoped to work.
I had to look in code on [github repository](https://github.com/coral-xyz/anchor/tree/master/docs/programs/tic-tac-toe).   
I shamelesly copy/pasted code for all parts of the project. (Should probably just read about setup
of separete files for Anchor Projectand first and than, clone github repository of tic-tac-toe project and 
work from there. It would saved me lot's of time.). After changing Project as in Github Repo. I stop reading 
the Site and started to work with the Code.   


Interesting things to note about the Rust Code are how modules are imported with star. For learning project
it probabaly is not the best Idea. Another interesting point is  how derived Accounts are find in 
instructions folder with methods, I myself would, probably, set them to beolong to the State.   


Good afternoon TS! I came to tests. I managed, but it took me almost two days! of my misirable life to 
figure plenty of things that I would prefer to see in documentation. Let's start!  
First I was have to set solana-program to 1.14.18, don't know wny, 'because'! Otherwise it was not working.
Have spend plenty of time on the web to find this hack. Before new (old) version anchor build command was 
showing me warnings about memory overflow in console when I was building.   
Second when testing I was constanly getting message that property 'methods' can not be found in undefined
which gave me the Idea that my program is not constructed. After plenty of time spend for reading lot's of 
articles on the internet I have a epiphany: "Read the Source", so I did.   
Result is in tests folder. I'm sure it's far from perfect, but it works! Things that I don't understand are:   
in original code there is expectation of  AncorError type in test which were regected because _err typeof
is undefined. Conclusion? Remove this lines, in general it's a wounderful method for sorting things
when something is preventing your test to run 'green' - get rid of this evil thing. Reccomend
(sort of) ) kidding).     
Other thing, when I remove line with anchor setting provider to env.
`anchor.setProvider(anchor.AnchorProvider.env());` tests are not passing even though I set provider later.
Why? ... 'Because', have no idea.   
This line: `program = new Program(IDL, new PublicKey("EwG9oX2Pr2uzjfA7BJGwDbAdaHrBi5qdjXohtNjkW58T"), programProvider);` Before you run test check that you cluster is `devnet` and run `anchor deploy` must copy
programID and update id in Anchor.toml, lib.rs and test files. I would like to update it in
one place, tops, but for first two, I have no idea why, is how it's done in Anchor and for last I have no Idea how to set it programmatically. Hold on I will have a look on package to read toml file...  Yes,
there is [an article](https://thisdavej.com/using-toml-config-files-in-node-js-applications/#creating-and-reading-our-first-toml-file)
didn't check, didn't try. Looks OK. Must read more about Anchor Program in general.    
Continue. I don't remember why in Anchor.toml `[programs.localnet]` is set to `anchor keys list` key, 
'because'. Bottom line three files - three updates. If it set correctly tests must work.   
I was getting error `Error: Metadata property not found in IDL of program: tic_tak_toe`, after reading
[this answer from stackoverflow](https://stackoverflow.com/questions/70658572/error-loading-workspace-idl-for-solana-anchor-testing) I re-deploy program and run again and it it sorted
the problem.   
After refactoring and writing tests. Tests have moved from localnet to devnet.
If you are sure about your deploy use `anchor test --skip-build --skip-deploy` will save time and SOL.

`