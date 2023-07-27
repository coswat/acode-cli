const create = require("./creater.js");
const docs = require("./docs.js");
const shellExec = require("./shellExec.js");

// get the argument from
let command = process.argv[2];

// index message
if (!command) {
  console.log(`
Acode Plugin CLI v 1.0.0 ( MIT )

Usage:

command [options] [arguments]

Available commands:

create             create a acode plugin template
docs               open plugin docs
build              alternative of npm run build
build-release      alternative of npm run build-release
`);
  process.exit(0);
}

// run the corresponding command with its name
switch (command) {
  case "create":
    create();
    break;
  case "docs":
    docs();
    break;
  case "build":
    process.chdir(process.cwd());
    shellExec("npm run build", false);
    break;
  case "build-release":
    process.chdir(process.cwd());
    shellExec("npm run build-release", false);
    break;
  default:
    // when command not found
    console.error("Invalid command " + command);
}
