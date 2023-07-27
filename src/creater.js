const inquirer = require("inquirer");
const Listr = require("listr");
const prompts = require("./createrPrompts.js");
const shellExec = require("./shellExec.js");
const fs = require("fs");
require("dotenv").config();

// Acode plugin url ( Js version )
const jsPlugin = process.env.JS_TEMPLATE_URL;
// Acode plugin url ( Ts version )
const tsPlugin = process.env.TS_TEMPLATE_URL;

// Main function
async function createAcodePlugin() {
  // questions to ask
  const questions = prompts;
  try {
    const answers = await inquirer.prompt(questions);
    const { projectName, language, usePrettier, useGit, installDep } = answers;
    // current directory
    const currentDir = process.cwd();
    // tasks to run
    const tasks = new Listr([
      {
        title: "Cloning plugin template",
        task: () => {
          process.chdir(currentDir);
          if (language === "TypeScript") {
            shellExec(`git clone ${tsPlugin} ${projectName}`);
          } else {
            shellExec(`git clone ${jsPlugin} ${projectName}`);
          }
        },
      },
      {
        title: "CleanUps",
        task: () => {
          process.chdir(currentDir + "/" + projectName);
          shellExec("rm -rf .git");
        },
      },
      {
        title: "Configuring Prettier",
        task: async () => {
          if (usePrettier) {
            await addPrettier();
          }
        },
      },
      {
        title: "Initializing Git repository",
        task: () => {
          if (useGit) {
            shellExec(
              'git init && git add . && git commit -m "Initial Commit"'
            );
          }
        },
      },
      {
        title: "Installing npm dependencies",
        task: () => {
          if (installDep) {
            shellExec("npm install");
          }
        },
      },
    ]);
    // Creating template messgae
    console.log(`Creating Acode plugin '${projectName}' with ${language}...`);
    // running all the tasks
    await tasks.run();
    // Success message
    console.log(
      `Acode plugin '${projectName}' with ${language} successfully created!`
    );
  } catch (error) {
    // Error handling
    console.error("Error occurred:", error.message);
    process.exit(1);
  }
}
// add prettier to package.json
async function addPrettier() {
  // locate the package.json path
  const packageJsonPath = process.cwd() + "/package.json";
  // require the file
  const packageJson = require(packageJsonPath);
  // add prettier package as a dev dependency
  packageJson.devDependencies = {
    ...packageJson.devDependencies,
    prettier: "3.0.0",
  };
  // json stringify the updated variable
  const updatedPackageJson = JSON.stringify(packageJson, null, 2);
  // update the original package.json file
  fs.writeFileSync(packageJsonPath, updatedPackageJson, "utf8");
}
// export the creator function
module.exports = createAcodePlugin;
