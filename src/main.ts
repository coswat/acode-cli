const create = require("./creater");
const docs = require("./docs");
import shellExec from "./shellExec";
import updateVersion from "./versionUpdater";
const { Command } = require("commander");
const helper = require("./helper");

// load the env file
helper.loadEnv();

// version
let version: string | undefined = process.env.VERSION;

// create a new command instance
const program = new Command();

program
  .name("acode-cli")
  .description(`Acode Plugin CLI ${version} (MIT) `)
  .version(`${version}`);
// Create Command
program
  .command("create")
  .description("Create a acode plugin template")
  .action(async () => {
    await create();
  });
// Docs Command
program
  .command("docs")
  .description("Open the plugin docs")
  .action(async () => {
    await docs();
  });
// Build Command
program
  .command("build")
  .description("Alternative of npm run build")
  .action(async () => {
    process.chdir(process.cwd());
    await shellExec("npm run build", false);
  });
// Build Release Command
program
  .command("build-release")
  .description("Alternative of npm run build-release")
  .action(async () => {
    process.chdir(process.cwd());
    await shellExec("npm run build-release", false);
  });
// Version updater Command
program
  .command("version <type>")
  .description("Update the plugin version")
  .action(async (type: string) => {
    await updateVersion(type);
  });
// Source Command
program
  .command("src")
  .description("Acode cli source code")
  .action(() => {
    console.log(`Source code url : ${process.env.SRC_CODE}`);
  });

program.parse(process.argv);
