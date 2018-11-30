# holochat-rust

[![Project](https://img.shields.io/badge/project-holochain-blue.svg?style=flat-square)](http://holochain.org/)
[![Chat](https://img.shields.io/badge/chat-chat%2eholochain%2enet-blue.svg?style=flat-square)](https://chat.holochain.net)
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

[![Code Status](https://img.shields.io/badge/Code-Pre--Alpha-orange.svg)](https://github.com/Holochain/holochat#feature-roadmap-and-current-progress)

***Multi-room P2P chat on Holochain**

**[Code Status:](https://github.com/holochain/holochain/milestones?direction=asc&sort=completeness&state=all)** Pre-alpha. Not for production use. This application has not been audited for any security validation.

## Install

1. Install the Holochain command line dev tool by following the instructions here: https://developer.holochain.org/start.html

2. Clone this repo:
```shell
    git clone https://github.com/holochain/holochat-rust
```

3. Make sure things are working by running the tests:

```shell
cd holochat-rust
cd test && npm install && cd ..
hc test | test/node_modules/faucet/bin/cmd.js
```

Note that by using the " | test/node_modules/faucet/bin/cmd.js" you lose the `console.log` output of your tests, but gain nice color coding.
If you want to see the logs, just use `hc test`.

4. Compile the DNA and run it using `hc` with:

```shell
hc run --port 3400 --package
```

Finally to run the holochat UI, simply open the `ui/index.html` file in a browser, and it should start communicating with the `hc` via websockets.

## Contribute
Holochain is an open source project.  We welcome all sorts of participation and are actively working on increasing surface area to accept it.  Please see our [contributing guidelines](https://github.com/holochain/org/blob/master/CONTRIBUTING.md) for our general practices and protocols on participating in the community.

## License
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

Copyright (C) 2018, Holochain Trust

This program is free software: you can redistribute it and/or modify it under the terms of the license p
rovided in the LICENSE file (GPLv3).  This program is distributed in the hope that it will be useful, bu
t WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
 PURPOSE.

**Note:** We are considering other 'looser' licensing options (like MIT license) but at this stage are using GPL while we're getting the matter sorted out.  See [this article](https://medium.com/holochain/licensing-needs-for-truly-p2p-software-a3e0fa42be6c) for some of our thinking on licensing for distributed application frameworks.
