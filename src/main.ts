const create = require("./creater");
const docs = require("./docs");
import shellExec from "./shellExec";
const { Command } = require("commander");
require("dotenv").config();

// version
let version: string | undefined = process.env.VERSION;

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
  .action(async () => {
    process.chdir(process.cwd());
    await shellExec("npm run build", false);
  });
program
  .command("build-release")
  .description("Alternative of npm run build-release")
  .action(async () => {
    process.chdir(process.cwd());
    await shellExec("npm run build-release", false);
  });
program
  .command("src")
  .description("Acode cli source code")
  .action(() => {
    console.log(`Source code url : ${process.env.SRC_CODE}`);
  });
program.parse();
