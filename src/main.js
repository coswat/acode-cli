const create = require("./creater.js");
const docs = require("./docs.js");
const shellExec = require("./shellExec.js");
const { Command } = require("commander");
require("dotenv").config();

// version
let version = process.env.VERSION;

// create a new command instance
const program = new Command();

program
  .name("acode-cli")
  .description(`Acode Plugin CLI ${version} (MIT) `)
  .version(`${version}`);
program
  .command("create")
  .description("Create a acode plugin template")
  .action(async () => {
    await create();
  });
program
  .command("docs")
  .description("Open the plugin docs")
  .action(async () => {
    await docs();
  });
program
  .command("build")
  .description("Alternative of npm run build")
  .action(() => {
    process.chdir(process.cwd());
    shellExec("npm run build", false);
  });
program
  .command("build-release")
  .description("Alternative of npm run build-release")
  .action(() => {
    process.chdir(process.cwd());
    shellExec("npm run build-release", false);
  });
program.parse();
