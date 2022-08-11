# GameLand
Gameland is an NFT rental platform and an immersive gaming community that connects developers with their players.


## High-Level Concept (Abstract)
Gameland as a Gamefi platform aims to help users experience high-end games at the lowest price, provide users a rental platform with integrating experience and revenue, and lower the boundary between players and games。

This multi-dimensional enriches the blockchain game itself and brings new upgrades to the blockchain game


## Our Vision 
Gameland provides an open game community, and any user can expose the NFT assets of their personal homepage to other users and realize leasing at the same time.

Game reviews are a feedback that we think is very important to game developers, we provide a comprehensive community for the game, and attract more attention and feedback from game users for the game. Compared with the game developer's separate community, which has more exposure and user participation, we provide an appreciation mechanism where game developers or users can give praise for high-quality feedback. Game developers can use the game's rating as a reference for players' evaluation of the game, or they can find user needs and suggestions from the comments to optimize, and can also use the game community to conduct community operations to attract some loyal gamers.
For game certification, there are many imitated game nft assets in the market, which are difficult for ordinary users to identify. When we select each project, we confirm the asset smart contract of each game. Therefore, users can clearly distinguish the authenticity of the game through the Gameland community.
The Enlightenment Plan is to meet the demands of developers "to meet players as soon as possible in the early stages of game development". Players have a strong demand for the game community, and the channel + vertical community model can give players a one-stop high-quality experience. Build a bridge for developers to connect with users, and solve the R&D pain point that it is difficult for developers to obtain high-quality feedback from players. Developers can verify the game design at an early stage and reduce development risks.

## Technology Stack
* Client Framework - React + Typescript 
* Ethereum development environment - Solidity + Hardhat
* Ethereum Web Client Library - Web3.js&Truffle/Ether.js&Waffle

## The process works as follow:
* Lenders fill up the name, nature of the item and upload the image of it.
* Gameland will then use the information above as a file and send it to IPFS to generate a CID
* The CID and the lender address will be the parameters to call our ERC721 contract to mint a NFT
* To call the functions at Gameland on UI, the NFT ID will be used to fetch the corresponding CID and the showcase will be rendered

## Tentative Milestones
*  UI building (1week) & Testing
*  Testnet launch(2weeks) : including lending, borrowing, withdraw and liquidation etc. 
*  Bugs fixing and function optimizations(2weeks): imposing a fine as penalty instead of confiscating the collaterals

